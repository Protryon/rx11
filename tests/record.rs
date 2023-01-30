use rx11::{
    coding::{Reply, RequestBody},
    events::{Event, EventCode},
    net::{X11Connection, X11ErrorCode},
    requests::{ClientSpec, ElementHeader, RecordTarget, RecordingReceiver, ReplyMetadata, RequestMetadata, Timestamp},
};

struct PrintReceiver;

impl RecordingReceiver for PrintReceiver {
    fn client_started(&mut self, client: ClientSpec, server_time: Timestamp) {
        println!("{:016}: client started {client:?}", server_time.0);
    }

    fn client_died(&mut self, client: ClientSpec, server_time: Timestamp, _last_sequence: Option<u32>) {
        println!("{:016}: client died {client:?}", server_time.0);
    }

    fn request(&mut self, major_opcode: u8, minor_opcode: u8, request: RequestBody, metadata: RequestMetadata) {
        println!(
            "{:016}{{{:05}}}<{:?}>: request {major_opcode}/{minor_opcode}: {request:?}",
            metadata.intercept_server_time.0,
            metadata.request_sequence.unwrap_or_default(),
            metadata.client
        );
    }

    fn reply(&mut self, reply: Reply, metadata: ReplyMetadata) {
        println!("{:016}{{{:05}}}<{:?}>: reply: {reply:?}", metadata.intercept_server_time.0, reply.sequence_number, metadata.client);
    }

    fn event(&mut self, event: Event, metadata: ReplyMetadata) {
        println!("{:016}<{:?}>: event: {event:?}", metadata.intercept_server_time.0, metadata.client);
    }

    fn error(&mut self, error: X11ErrorCode, _value: u32, sequence: u32, metadata: ReplyMetadata) {
        println!("{:016}{{{:05}}}<{:?}>: error: {error:?}", metadata.intercept_server_time.0, sequence, metadata.client);
    }
}

#[tokio::test]
async fn test_x11() {
    env_logger::Builder::new()
        .parse_env(env_logger::Env::default().default_filter_or("info"))
        .init();
    let connected = X11Connection::connect("", 1).await.expect("failed to connect to x11");
    let connected2 = connected.clone();
    tokio::spawn(async move {
        let mut receiver = connected2.events();
        while let Some(event) = receiver.recv().await {
            let event = event.unwrap();
            println!("event: {:?}", event);
        }
    });

    let context = connected
        .create_record_context(
            ElementHeader::ALL,
            [ClientSpec::AllClients],
            [
                RecordTarget::DeviceCoreEvent(EventCode::KeyPress),
                RecordTarget::DeviceCoreEvent(EventCode::KeyRelease),
            ],
        )
        .await
        .unwrap();
    context.enable(&mut PrintReceiver).await.unwrap();
    connected.log_errors().await;
}
