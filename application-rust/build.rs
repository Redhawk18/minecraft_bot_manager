use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(
        &[
            "../proto/baritone.proto",
            "../proto/commands.proto",
            "../proto/common.proto",
            "../proto/connection.proto",
            "../proto/chat.proto",
            "../proto/inventory.proto",
            "../proto/meteor.proto",
            "../proto/player.proto",
            "../proto/protocol.proto",
        ],
        &["../proto/"],
    )?;

    Ok(())
}
