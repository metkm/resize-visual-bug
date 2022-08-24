use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder, menu::MenuBar,
    },
    webview::WebViewBuilder,
};

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();

    let mut menu = MenuBar::new();
    menu.add_native_item(wry::application::menu::MenuItem::Quit);

    let window = WindowBuilder::new()
        .with_title("Hello World")
        .with_menu(menu)
        .build(&event_loop)?;

    let _webview = WebViewBuilder::new(window)?
        .with_html("<p>Hello World!</p>")?
        .build()?;

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
