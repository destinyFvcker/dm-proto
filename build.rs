use prost_build;

fn main() {
    prost_build::Config::new()
        .compile_protos(&["proto/kafka_alarm.proto", "proto/ws.proto"], &["proto/"])
        .unwrap();
}
