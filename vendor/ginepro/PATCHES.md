# Local ginepro patch

Source: https://github.com/gkumbhat/ginepro/tree/49e56eada1cfda8af206383e014fb0ce4641da25

The library sources and licenses are copied from the pinned dependency. Local changes:

- Require hickory-resolver 0.26.3 to fix CVE-2026-93657 (fixed in 0.26.2;
  0.26.3 also fixes regressions introduced in 0.26.2).
- Adapt resolver construction to the 0.26 API, preserving system DNS settings,
  disabled caching, IPv4-first lookup strategy, and error propagation.
- Use the new path for Hickory's DNS name type.
- Adjust the README path and omit upstream workspace-only dev dependencies.

Remove this patch when the fork supports the patched Hickory release with tonic
0.14 and the endpoint settings used by the orchestrator.
