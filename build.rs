fn main() {
    // Use the `tonic_build` crate to configure code generation
    tonic_build::configure()
        .build_server(true) // Generate server code
        .out_dir("src/")    // Specify the output directory for generated code
        .compile(
            &["proto/zkp_auth.proto"], // Specify the protobuf file to compile
            &["proto/"],               // Specify the root location to search for proto dependencies
        )
        .unwrap(); // Handle potential errors during code generation
}
