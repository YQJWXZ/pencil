fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = vec![
        "../protos/common/common.proto",
        "../protos/auth/auth.proto",
        "../protos/article/article.proto",
        "../protos/asset/asset.proto",
    ];

    // Use tonic-build with prost for code generation
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&proto_files, &["../protos"])?;

    Ok(())
}
