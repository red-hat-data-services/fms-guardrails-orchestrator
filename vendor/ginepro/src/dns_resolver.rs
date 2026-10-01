//! Implements [`LookupService`] for dns.

use crate::{LookupService, ServiceDefinition};
use anyhow::Context;
use hickory_resolver::config::LookupIpStrategy;
use hickory_resolver::TokioResolver;
use std::collections::HashSet;
use std::net::SocketAddr;

/// Implements [`LookupService`] by using DNS queries to lookup [`ServiceDefinition::hostname`].
pub struct DnsResolver {
    /// The trust-dns resolver which contacts the dns service directly such
    /// that we bypass os-specific dns caching.
    dns: TokioResolver,
}

impl DnsResolver {
    /// Construct a new [`DnsResolver`] from env and system configuration, e.g `resolv.conf`.
    pub async fn from_system_config() -> Result<Self, anyhow::Error> {
        let mut builder = TokioResolver::builder_tokio()
            .context("failed to read dns services from system configuration")?;

        // We do not want any caching on our side.
        builder.options_mut().cache_size = 0;
        // Preserve Hickory 0.25's lookup strategy.
        builder.options_mut().ip_strategy = LookupIpStrategy::Ipv4thenIpv6;

        let dns = builder.build()?;

        Ok(Self { dns })
    }
}

#[async_trait::async_trait]
impl LookupService for DnsResolver {
    #[tracing::instrument(level = "debug", skip(self))]
    async fn resolve_service_endpoints(
        &self,
        definition: &ServiceDefinition,
    ) -> Result<HashSet<SocketAddr>, anyhow::Error> {
        match self.dns.lookup_ip(definition.hostname()).await {
            Ok(lookup) => {
                tracing::debug!("dns query expires in: {:?}", lookup.valid_until());
                Ok(lookup
                    .iter()
                    .map(|ip_addr| {
                        tracing::debug!("result: ip {}", ip_addr);
                        (ip_addr, definition.port()).into()
                    })
                    .collect())
            }
            Err(err) => Err(err.into()),
        }
    }
}
