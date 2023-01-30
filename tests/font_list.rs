use rx11::net::X11Connection;

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

    let fonts = connected.list_fonts_with_info(u16::MAX, "*").await.unwrap();
    println!("fonts = {fonts:?}");

    connected.log_errors().await;
}
