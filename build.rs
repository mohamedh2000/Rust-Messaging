fn main() {
    // compiling protos using path on build time
    tonic_build::compile_protos("proto/chat.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
    
}