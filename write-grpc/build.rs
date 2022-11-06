fn main() {
    tonic_build::compile_protos("proto/channel/channel.proto").unwrap();
    tonic_build::compile_protos("proto/message/message.proto").unwrap();
}
