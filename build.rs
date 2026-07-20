use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SAFETY: build scripts are single-threaded; no concurrent env access.
    unsafe { std::env::set_var("PROTOC", protobuf_src::protoc()) };
    fs::create_dir("src/pb").unwrap_or(());
    tonic_prost_build::configure()
        .build_client(true)
        .build_server(false) // Only clients needed
        .out_dir("src/pb")
        .include_file("mod.rs")
        .compile_protos(
            &[
                "protos/caikit_runtime_Chunkers.proto",
                "protos/caikit_runtime_Nlp.proto",
                "protos/generation.proto",
                "protos/caikit_data_model_caikit_nlp.proto",
                "protos/health_check.proto",
            ],
            &["protos"],
        )
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {e}"));

    Ok(())
}
