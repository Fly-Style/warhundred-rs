fn main() {
    println!("Running Prost build...");

    let mut prost_build_config = prost_build::Config::new();

    prost_build_config.out_dir("./src/app/protos");

    prost_build_config
        .compile_protos(
            &["./src/app/protos/messages.proto"],
            &["./src/common/protos/"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos: {:?}", e));
}
