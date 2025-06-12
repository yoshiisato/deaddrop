fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
    .protoc_arg("--experimental_allow_proto3_optional")
    .out_dir("./src")
    .compile(&["./proto/common.proto"], &["."])?;
    Ok(())
}