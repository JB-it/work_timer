use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{WindowBuilder, Window, WindowLevel}, dpi::{PhysicalPosition, LogicalSize},
};

use tray_icon::{TrayIconBuilder, menu::Menu};

fn main() {
    let tray_menu = Menu::new();

    let path = "./assets/icon.png";
    let icon = load_icon(std::path::Path::new(path));

    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("system-tray - tray icon library!")
        .with_icon(icon)
        .build()
        .unwrap();

    let event_loop = EventLoop::new();
    
    let window: Window = WindowBuilder::new()
        .with_decorations(false)
        .with_transparent(true)
        .with_inner_size(LogicalSize::new(200, 200))
        .with_position(PhysicalPosition::new(600, 100))
        .with_window_level(WindowLevel::AlwaysOnTop)
        .with_title("A fantastic window!")
        .build(&event_loop)
        .unwrap();


    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();
        //println!("{event:?}");



        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => control_flow.set_exit(),
            _ => (),
        }
    });
}

//Copied from https://github.com/tauri-apps/tray-icon/blob/dev/examples/winit.rs
fn load_icon(path: &std::path::Path) -> tray_icon::icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::icon::Icon::from_rgba(icon_rgba, icon_width, icon_height)
        .expect("Failed to open icon")
}