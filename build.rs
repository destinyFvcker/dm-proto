use prost_build;

fn main() {
    prost_build::Config::new()
        .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
        .compile_protos(&["proto/kafka_alarm.proto", "proto/ws.proto"], &["proto/"])
        .unwrap();
}
