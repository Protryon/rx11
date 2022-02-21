use std::time::Duration;

use rx11::{connection::X11Connection, requests::{WindowParamsBuilder, Atom, GContextParamsBuilder, Rectangle, WindowAttributesBuilder, EventMask}, events::Event};


#[tokio::test]
async fn test_x11() {
    env_logger::Builder::new()
        .parse_env(env_logger::Env::default().default_filter_or("info"))
        .init();
    let connected = X11Connection::connect("127.0.0.1", 1).await.expect("failed to connect to x11");
    // println!("{:?}", connected.handshake());
    let connected2 = connected.clone();
    tokio::spawn(async move {
        let mut receiver = connected2.events();
        while let Some(event) = receiver.recv().await {
            match &event {
                Event::PropertyNotify(n) => {
                    let name = n.name.resolve(&connected2).await.unwrap();
                    println!("property name = {}", name);
                },
                _ => (),
            }
            println!("event: {:?}", event);
        }
    });
    let extensions = connected.list_extensions().await.unwrap();
    for extension in &extensions {
        let query = connected.query_extension(&**extension).await.unwrap();
        println!("{}: {:?}", extension, query);
    }
    let window = connected.create_window(WindowParamsBuilder::default()
        .attributes(WindowAttributesBuilder::default()
            .event_mask(
                EventMask::KEY_PRESS |
                EventMask::KEY_RELEASE |
                EventMask::BUTTON_PRESS |
                EventMask::BUTTON_RELEASE |
                EventMask::ENTER_WINDOW |
                EventMask::LEAVE_WINDOW |
                EventMask::POINTER_MOTION |
                EventMask::BUTTON1_MOTION |
                EventMask::BUTTON2_MOTION |
                EventMask::BUTTON3_MOTION |
                EventMask::BUTTON4_MOTION |
                EventMask::BUTTON5_MOTION |
                EventMask::BUTTON_MOTION | 
                EventMask::KEYMAP_STATE |
                EventMask::EXPOSURE |
                EventMask::VISIBILITY_CHANGE |
                EventMask::STRUCTURE_NOTIFY |
                EventMask::SUBSTRUCTURE_NOTIFY |
                EventMask::FOCUS_CHANGE |
                EventMask::PROPERTY_CHANGE |
                EventMask::COLORMAP_CHANGE |
                EventMask::OWNER_GRAB_BUTTON
            )
            // EventMask::ALL & !EventMask::POINTER_MOTION_HINT
            .build().unwrap()
        )
        .build().unwrap()
    ).await.unwrap();
    window.set_property_string(Atom::WM_NAME, "test title").await.unwrap();
    let gcontext = connected.create_gcontext(window, GContextParamsBuilder::default().foreground(0xff0000).build().unwrap()).await.unwrap();

    window.map().await.unwrap();
    connected.log_errors().await;
    let mut counter = 0;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        counter += 1;
        gcontext.poly_fill_rectangle(window, vec![
            Rectangle { x: 50, y: 50, width: 100, height: 100 },
        ]).await.unwrap();
    
        window.set_property_string(Atom::WM_NAME, format!("test title {}", counter)).await.unwrap();

        connected.log_errors().await;
    }
}