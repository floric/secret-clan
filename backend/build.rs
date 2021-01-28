use protoc_rust::Customize;

extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/model/proto")
        .inputs(&[
            "../schema/message.proto",
            "../schema/task.proto",
            "../schema/message.proto",
            "../schema/player.proto",
            "../schema/game.proto",
            "../schema/role.proto",
            "../schema/vote.proto",
        ])
        .include("../schema")
        .customize(Customize {
            expose_fields: Some(true),
            ..Default::default()
        })
        .run()
        .expect("protoc has failed");
}
