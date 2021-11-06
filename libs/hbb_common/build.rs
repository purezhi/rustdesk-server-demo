use protobuf_codegen::Codegen;

fn main() {
    std::fs::create_dir_all("src/protos").unwrap();
    Codegen::new()
        .pure()
        .out_dir("src/protos")
        .inputs(&["protos/rendezvous.proto", "protos/message.proto"])
        .include("protos")
        .run()
        .expect("Codegen failed.");
}
