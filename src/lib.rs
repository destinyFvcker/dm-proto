pub mod kafka_alarm {
    include!(concat!(env!("OUT_DIR"), "/dm_proto.kafka_alarm.rs"));
}

pub mod ws {
    include!(concat!(env!("OUT_DIR"), "/dm_proto.ws.rs"));
}
