use std::{thread::sleep, time::Duration};

use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder, menu::MenuBar, platform::windows::WindowExtWindows,
    },
    webview::WebViewBuilder,
};

use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::SetMenu
};

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();

    let mut menu = MenuBar::new();
    menu.add_native_item(wry::application::menu::MenuItem::Quit);

    let window = WindowBuilder::new()
        .with_title("Hello World")
        .with_menu(menu)
        .build(&event_loop)?;

    let hwnd: HWND = unsafe { std::mem::transmute(window.hwnd()) };

    let _webview = WebViewBuilder::new(window)?
        .with_html("<p>Hello World!</p>")?
        .build()?;

    // Code to remove menu after a while
    std::thread::spawn(move || {
        sleep(Duration::from_secs(5));
        println!("Removing the menu!");

        unsafe {
            SetMenu(hwnd, None);
        }
    });
    //

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
