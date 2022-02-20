use std::time::Duration;

use rx11::{connection::X11Connection, requests::{WindowParamsBuilder, Atom, GContextParamsBuilder}, coding::Rectangle};


#[tokio::test]
async fn test_x11() {
    env_logger::Builder::new()
        .parse_env(env_logger::Env::default().default_filter_or("info"))
        .init();
    let connected = X11Connection::connect("127.0.0.1", 1, Box::new(|e| {
        log::info!("event: {:?}", e);
        Ok(())
    })).await.expect("failed to connect to x11");
    println!("{:?}", connected.handshake());
    let extensions = connected.list_extensions().await.unwrap();
    for extension in &extensions {
        let query = connected.query_extension(&**extension).await.unwrap();
        println!("{}: {:?}", extension, query);
    }
    let window = connected.create_window(WindowParamsBuilder::default().build().unwrap()).await.unwrap();
    connected.set_property_string(window, Atom::WM_NAME, "test title").await.unwrap();
    let gcontext = connected.create_gcontext(window, GContextParamsBuilder::default().foreground(0xff0000).build().unwrap()).await.unwrap();

    connected.map_window(window).await.unwrap();
    connected.log_errors().await;
    let mut counter = 0;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        counter += 1;
        connected.poly_fill_rectangle(window, gcontext, vec![
            Rectangle { x: 50, y: 50, width: 100, height: 100 },
        ]).await.unwrap();
    
        connected.set_property_string(window, Atom::WM_NAME, format!("test title {}", counter)).await.unwrap();
        connected.log_errors().await;
    }
}