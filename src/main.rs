mod experimental;

use gtk4::prelude::*;
use libadwaita::prelude::*;

use libadwaita::Application;
use libadwaita::ApplicationWindow as Window;

fn main() {
    // Create a new Adw application.
    let app = libadwaita::Application::builder()
        .application_id("io.github.stellarst0rm.whiteboard")
        .build();

    app.connect_activate(app_closure);
    app.run();
}

fn app_closure(app: &Application) {
    // Build the window
    let window = Window::builder()
        .application(app)
        .title("Whiteboard")
        .build();

    window.maximize();

    // Build the UI
    let builder = gtk4::Builder::from_string(include_str!("structure.ui"));

    let overlay: gtk4::Overlay = builder.object("overlay").unwrap();
    let whiteboard: gtk4::DrawingArea = builder.object("whiteboard").unwrap();
    let toolbox: gtk4::Box = builder.object("toolbox").unwrap();
    let toolbox_wrapper: gtk4::ListBox = builder.object("toolbox_wrapper").unwrap();

    overlay.add_overlay(&toolbox_wrapper);

    for i in 1..=4 {
        let button = gtk4::Button::with_label(&format!("Button {}", i));

        toolbox.append(&button);
    }

    experimental::setup_draw(&whiteboard);

    // Make the window background semi-transparent
    let css = gtk4::CssProvider::new();
    css.load_from_data(include_str!("custom.css"));
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().unwrap(),
        &css,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    window.set_content(Some(&overlay));
    window.present();
}
