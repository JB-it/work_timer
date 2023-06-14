use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::{WindowBuilder, Window, WindowLevel}, dpi::{PhysicalPosition, LogicalSize},
};

use tray_icon::{TrayIconBuilder, menu::{Menu, MenuItem, MenuEvent}, TrayEvent};

enum MenuAction {
    Exit,
    None
}

fn main() {
    let path = "./assets/icon.png";
    let icon = load_icon(std::path::Path::new(path));
    

    let item1 = MenuItem::new("Item 1", true, None);
    let item2 = MenuItem::new("Item 2", true, None);
    let exit_item = MenuItem::new("Exit", true, None);
    let tray_menu = Menu::new();
    tray_menu.append(&item1);
    tray_menu.append(&item2);
    tray_menu.append(&exit_item);

    //A dictionary of Strings and the corresponding MenuItem ids
    let menu_dict: Vec<(MenuAction, u32)> = vec![
        (MenuAction::None, item1.id()), 
        (MenuAction::None, item2.id()), 
        (MenuAction::Exit, exit_item.id())
    ];

    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Work Timer Settings")
        .with_icon(icon)
        .build()
        .unwrap();

    let event_loop = EventLoop::new();
    
    let _window: Window = WindowBuilder::new()
        .with_decorations(false)
        .with_transparent(true)
        .with_inner_size(LogicalSize::new(50, 50))
        .with_position(PhysicalPosition::new(50, 920))
        .with_window_level(WindowLevel::AlwaysOnTop)
        .with_title("Work timer")
        .build(&event_loop)
        .unwrap();


    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayEvent::receiver();

    let mut i = 0;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        //println!("{}", i);
        //i += 1;

        if let Ok(event) = tray_channel.try_recv() {
            println!("{event:?}");
        }

        if let Ok(event) = menu_channel.try_recv() {
            println!("{event:?}");
        }

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