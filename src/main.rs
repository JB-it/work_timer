use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{WindowBuilder, Window, WindowLevel}, dpi::{PhysicalPosition, LogicalSize},
};
use std::{collections::HashMap, num::NonZeroU32};
use tray_icon::{TrayIconBuilder, menu::{Menu, MenuItem, MenuEvent}, TrayEvent};

enum MenuAction {
    Exit,
    None
}

const WORK_SECONDS: i64 = 20 * 60; // 20 minutes
const BREAK_SECONDS: i64 = 5 * 60; // 5 minutes

const RED: u32 = 0xFFFF0000;
const GREEN: u32 = 0xFF00FF00;

enum WorkState {
    Working,
    Break
}

fn main() {
    let path = "./assets/icon.png";
    let icon = load_icon(std::path::Path::new(path));
    

    //let item1 = MenuItem::new("Item 1", true, None);
    //let item2 = MenuItem::new("Item 2", true, None);
    let exit_item = MenuItem::new("Exit", true, None);
    let tray_menu = Menu::new();
    //tray_menu.append(&item1);
    //tray_menu.append(&item2);
    tray_menu.append(&exit_item);

    let mut menu_dict: HashMap<u32, MenuAction> = HashMap::new();
    //menu_dict.insert(item1.id(), MenuAction::None);
    //menu_dict.insert(item2.id(), MenuAction::None);
    menu_dict.insert(exit_item.id(), MenuAction::Exit);

    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Work Timer Settings")
        .with_icon(icon)
        .build()
        .unwrap();

    let event_loop = EventLoop::new();
    
    let window: Window = WindowBuilder::new()
        .with_decorations(false)
        .with_transparent(false)
        .with_inner_size(LogicalSize::new(50, 50))
        .with_position(PhysicalPosition::new(50, 920))
        .with_window_level(WindowLevel::AlwaysOnTop)
        .with_title("Work timer")
        .build(&event_loop)
        .unwrap();

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();
    
    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayEvent::receiver();

    let mut start_time = chrono::offset::Local::now();

    let mut work_state = WorkState::Working;

    event_loop.run(move |event, _, control_flow| {
        //*control_flow = ControlFlow::Poll;

        let c_stamp = chrono::offset::Local::now().timestamp();
        let s_stamp = start_time.timestamp();

        match work_state {
            WorkState::Working => {
                if c_stamp - s_stamp > WORK_SECONDS {
                    work_state = WorkState::Break;
                    start_time = chrono::offset::Local::now();
                    window.request_redraw();
                    println!("Changed to Break State");
                }
            }
            WorkState::Break => {
                if c_stamp - s_stamp > BREAK_SECONDS {
                    work_state = WorkState::Working;
                    start_time = chrono::offset::Local::now();
                    window.request_redraw();
                    println!("Changed to Work State");
                }
            }
        }

        if let Ok(event) = tray_channel.try_recv() {
            println!("{event:?}");
        }

        if let Ok(event) = menu_channel.try_recv() {
            match menu_dict.get(&event.id).expect("Missing Menu item in dict") {
                MenuAction::Exit => {
                    println!("Exiting");
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                },
                MenuAction::None => {
                    println!("None");
                }
            }
            println!("{event:?}");
        }

        match event {
            Event::RedrawRequested(_window_id) => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();
                    let mut buffer = surface.buffer_mut().unwrap();
                    for index in 0..((width * height) as usize) {
                        buffer[index] = match work_state {
                            WorkState::Working => RED,
                            WorkState::Break => GREEN
                        };
                    }

                    buffer.present().unwrap();
            }
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