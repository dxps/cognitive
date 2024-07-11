use axum_connect_build::{axum_connect_codegen, AxumConnectGenSettings};

fn main() {
    // This helper will use `proto` as the import path, and globs all .proto
    // files in the `proto` directory.
    //
    // Note that you might need to re-save the `build.rs` file after updating
    // a proto file to get rust-analyzer to pickup the change. I haven't put
    // time into looking for a fix to that yet.
    let settings = AxumConnectGenSettings::from_directory_recursive("proto").expect("failed to glob proto files");

    axum_connect_codegen(settings).unwrap();
}
