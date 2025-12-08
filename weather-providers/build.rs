fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc_path = protoc_bin_vendored::protoc_bin_path().map_err(Box::new)?;

    unsafe {
        std::env::set_var("PROTOC", protoc_path);
    }

    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(&["proto/weather.proto"], &["proto"])?;

    Ok(())
}
