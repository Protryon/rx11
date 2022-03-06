
fn main() {
    protospec_build::compile_spec("x11", include_str!("./spec/x11.pspec"), &protospec_build::Options {
        include_async: true,
        use_anyhow: true,
        // debug_mode: true,
        ..Default::default()
    }).expect("failed to build x11.pspec");
    protospec_build::compile_spec("xkb", include_str!("./spec/xkb.pspec"), &protospec_build::Options {
        include_async: false,
        use_anyhow: true,
        // debug_mode: true,
        ..Default::default()
    }).expect("failed to build xkb.pspec");
    // protospec_build::compile_spec("xinput1", include_str!("./spec/xinput1.pspec"), &protospec_build::Options {
    //     include_async: false,
    //     use_anyhow: true,
    //     // debug_mode: true,
    //     ..Default::default()
    // }).expect("failed to build xinput1.pspec");
    protospec_build::compile_spec("xinput2", include_str!("./spec/xinput2.pspec"), &protospec_build::Options {
        include_async: false,
        use_anyhow: true,
        // debug_mode: true,
        ..Default::default()
    }).expect("failed to build xinput2.pspec");
}