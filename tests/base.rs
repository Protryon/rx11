use std::time::Duration;

use rx11::{
    net::X11Connection,
    requests::{
        Atom, DeviceSpec, EventMask, GBNDetail, GContextParamsBuilder, MapPart, NameDetail,
        Rectangle, SetOfGroup, StatePart, WindowAttributesBuilder, WindowParamsBuilder,
        XIEventMask, XKBEventsBuilder,
    },
};

#[tokio::test]
async fn test_x11() {
    env_logger::Builder::new()
        .parse_env(env_logger::Env::default().default_filter_or("info"))
        .init();
    let connected = X11Connection::connect("127.0.0.1", 1)
        .await
        .expect("failed to connect to x11");
    // println!("{:?}", connected.handshake());
    let connected2 = connected.clone();
    tokio::spawn(async move {
        let mut receiver = connected2.events();
        while let Some(event) = receiver.recv().await {
            let event = event.unwrap();
            println!("event: {:?}", event);
        }
    });
    let extensions = connected.list_extensions().await.unwrap();
    for extension in &extensions {
        let query = connected.query_extension(&**extension).await.unwrap();
        println!("{}: {:?}", extension, query);
    }
    let window = connected
        .create_window(
            WindowParamsBuilder::default()
                .attributes(
                    WindowAttributesBuilder::default()
                        .event_mask(
                            // EventMask::KEY_PRESS |
                            // EventMask::KEY_RELEASE |
                            // EventMask::BUTTON_PRESS |
                            // EventMask::BUTTON_RELEASE |
                            // EventMask::ENTER_WINDOW |
                            // EventMask::LEAVE_WINDOW |
                            // EventMask::POINTER_MOTION |
                            // EventMask::BUTTON1_MOTION |
                            // EventMask::BUTTON2_MOTION |
                            // EventMask::BUTTON3_MOTION |
                            // EventMask::BUTTON4_MOTION |
                            // EventMask::BUTTON5_MOTION |
                            // EventMask::BUTTON_MOTION |
                            // EventMask::KEYMAP_STATE |
                            EventMask::EXPOSURE |
                EventMask::VISIBILITY_CHANGE |
                EventMask::STRUCTURE_NOTIFY |
                EventMask::SUBSTRUCTURE_NOTIFY |
                // EventMask::FOCUS_CHANGE |
                EventMask::PROPERTY_CHANGE |
                EventMask::COLORMAP_CHANGE |
                EventMask::OWNER_GRAB_BUTTON,
                        )
                        // EventMask::ALL & !EventMask::POINTER_MOTION_HINT
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .await
        .unwrap();
    window
        .set_property_string(Atom::WM_NAME, "test title")
        .await
        .unwrap();
    let gcontext = connected
        .create_gcontext(
            window,
            GContextParamsBuilder::default()
                .foreground(0xff0000)
                .build()
                .unwrap(),
        )
        .await
        .unwrap();

    window.map().await.unwrap();
    connected.log_errors().await;

    let compatmap = connected
        .xkb_get_compat_map(DeviceSpec::UseCoreKeyboard, SetOfGroup::ALL, None)
        .await
        .unwrap();
    println!("compatmap = {:?}", compatmap);
    let indicator_map = connected
        .xkb_get_indicator_map(DeviceSpec::UseCoreKeyboard, u32::MAX)
        .await
        .unwrap();
    println!("indicator_map = {:?}", indicator_map);
    let keynames = connected
        .xkb_get_names(DeviceSpec::UseCoreKeyboard, NameDetail::KEY_NAMES)
        .await
        .unwrap();
    println!("keynames = {:?}", keynames);
    let othernames = connected
        .xkb_get_names(
            DeviceSpec::UseCoreKeyboard,
            NameDetail::ALL ^ NameDetail::KEY_NAMES,
        )
        .await
        .unwrap();
    println!("othernames = {:?}", othernames);
    let map = connected
        .xkb_get_map(DeviceSpec::UseCoreKeyboard, MapPart::ALL)
        .await
        .unwrap();
    println!("map = {:?}", map);
    // let gname = connected.get_atom_name(0xf9).await.unwrap();
    // let geometry = connected.xkb_get_geometry(DeviceSpec::UseCoreKeyboard, gname).await.unwrap();
    // println!("geometry = {:?}", geometry);
    let kbd = connected
        .xkb_get_keyboard_by_name(
            DeviceSpec::UseCoreKeyboard,
            GBNDetail::ALL,
            GBNDetail::ALL,
            false,
            "",
            "",
            "",
            "",
            "",
            "",
        )
        .await
        .unwrap();
    println!("keyboard = {:?}", kbd);

    connected
        .xkb_select_events(
            DeviceSpec::UseCoreKeyboard,
            XKBEventsBuilder::default()
                .state_notify(StatePart::ALL)
                .build()
                .unwrap(),
        )
        .await
        .unwrap();

    let devices = connected.all_xi_master_devices().query().await.unwrap();
    for device in devices {
        println!("master device = {:?}", device);
        let properties = device.device.list_properties().await.unwrap();
        println!("props = {:?}", properties);
    }

    let current_pointer = window
        .get_client_pointer()
        .await
        .unwrap()
        .expect("no client pointer");
    // let mask = XIEventMask::DEVICE_CHANGED | XIEventMask::BUTTON_PRESS | XIEventMask::BUTTON_RELEASE | XIEventMask::MOTION | XIEventMask::ENTER | XIEventMask::LEAVE | XIEventMask::FOCUS_IN | XIEventMask::FOCUS_OUT;
    let mask = XIEventMask::BUTTON_PRESS
        | XIEventMask::BUTTON_RELEASE
        | XIEventMask::MOTION
        | XIEventMask::ENTER;
    window
        .xi_select_events([(current_pointer, mask)])
        .await
        .unwrap();

    let mut counter = 0;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        counter += 1;
        gcontext
            .poly_fill_rectangle(
                window,
                vec![Rectangle {
                    x: 50,
                    y: 50,
                    width: 100,
                    height: 100,
                }],
            )
            .await
            .unwrap();

        window
            .set_property_string(Atom::WM_NAME, format!("test title {}", counter))
            .await
            .unwrap();

        connected.log_errors().await;
    }
}

/*
BIG-REQUESTS: QueryExtensionReply { present: true, major_opcode: 133, first_event: 0, first_error: 0 }
Generic Event Extension: QueryExtensionReply { present: true, major_opcode: 128, first_event: 0, first_error: 0 }
XKEYBOARD: QueryExtensionReply { present: true, major_opcode: 135, first_event: 85, first_error: 137 }
XInputExtension: QueryExtensionReply { present: true, major_opcode: 131, first_event: 66, first_error: 129 }
XFIXES: QueryExtensionReply { present: true, major_opcode: 138, first_event: 87, first_error: 140 }

SHAPE: QueryExtensionReply { present: true, major_opcode: 129, first_event: 64, first_error: 0 }
RANDR: QueryExtensionReply { present: true, major_opcode: 140, first_event: 89, first_error: 147 }

RECORD: QueryExtensionReply { present: true, major_opcode: 146, first_event: 0, first_error: 154 }
XINERAMA: QueryExtensionReply { present: true, major_opcode: 141, first_event: 0, first_error: 0 }
Composite: QueryExtensionReply { present: true, major_opcode: 142, first_event: 0, first_error: 0 }
RENDER: QueryExtensionReply { present: true, major_opcode: 139, first_event: 0, first_error: 142 }
MIT-SHM: QueryExtensionReply { present: true, major_opcode: 130, first_event: 65, first_error: 128 }

XTEST: QueryExtensionReply { present: true, major_opcode: 132, first_event: 0, first_error: 0 }
SYNC: QueryExtensionReply { present: true, major_opcode: 134, first_event: 83, first_error: 134 }
XC-MISC: QueryExtensionReply { present: true, major_opcode: 136, first_event: 0, first_error: 0 }
SECURITY: QueryExtensionReply { present: true, major_opcode: 137, first_event: 86, first_error: 138 }
XINERAMA: QueryExtensionReply { present: true, major_opcode: 141, first_event: 0, first_error: 0 }
DAMAGE: QueryExtensionReply { present: true, major_opcode: 143, first_event: 91, first_error: 152 }
MIT-SCREEN-SAVER: QueryExtensionReply { present: true, major_opcode: 144, first_event: 92, first_error: 0 }
DOUBLE-BUFFER: QueryExtensionReply { present: true, major_opcode: 145, first_event: 0, first_error: 153 }
DPMS: QueryExtensionReply { present: true, major_opcode: 147, first_event: 0, first_error: 0 }
Present: QueryExtensionReply { present: true, major_opcode: 148, first_event: 0, first_error: 0 }
X-Resource: QueryExtensionReply { present: true, major_opcode: 149, first_event: 0, first_error: 0 }
XVideo: QueryExtensionReply { present: true, major_opcode: 150, first_event: 93, first_error: 155 }
GLX: QueryExtensionReply { present: true, major_opcode: 151, first_event: 95, first_error: 158 }
XFree86-VidModeExtension: QueryExtensionReply { present: true, major_opcode: 152, first_event: 0, first_error: 172 }
XFree86-DGA: QueryExtensionReply { present: true, major_opcode: 153, first_event: 112, first_error: 179 }
DRI2: QueryExtensionReply { present: true, major_opcode: 154, first_event: 119, first_error: 0 }
NV-GLX: QueryExtensionReply { present: true, major_opcode: 155, first_event: 0, first_error: 0 }
NV-CONTROL: QueryExtensionReply { present: true, major_opcode: 156, first_event: 121, first_error: 0 }
*/
