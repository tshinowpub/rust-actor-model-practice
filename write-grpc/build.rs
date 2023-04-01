fn main() -> Result<(), Box<dyn std::error::Error>>{
    tonic_build::compile_protos("proto/channel/channel.proto")?;
    tonic_build::compile_protos("proto/message/message.proto")?;

    Ok(())
}
