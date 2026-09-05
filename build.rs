fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Vendored protoc, so no system protobuf-compiler installation is
    // required - the same pattern the rest of this ecosystem uses (engine,
    // uidgen, lending, eod, bot, crust, crudgrpc, json2grpc). Without it this
    // crate simply does not build on a machine that has no system `protoc`.
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    std::env::set_var("PROTOC", protoc);

    tonic_prost_build::configure()
        // This service is a *client* of engine and crudgrpc, never a server
        // of either - it serves its own HTTP webhook surface instead.
        .build_server(false)
        .build_client(true)
        .compile_protos(
            &["proto/engine.proto", "proto/crudgrpc.proto", "proto/uidgen.proto"], // Paths to your proto files
            &["proto"],                                      // The directory to look for imports
        )?;
    Ok(())
}
