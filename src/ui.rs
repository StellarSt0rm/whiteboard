use gtk4::prelude::*;
use libadwaita::ApplicationWindow as Window;
use libadwaita::prelude::*;

pub fn build(
    app: &libadwaita::Application,
) -> (
    Window,
    gtk4::Overlay,
    gtk4::DrawingArea,
    gtk4::Box,
    gtk4::ListBox,
) {
    let window = Window::builder()
        .application(app)
        .title("Whiteboard")
        .build();

    window.maximize();

    // Build the UI with GtkBuilder
    let builder = gtk4::Builder::from_string(include_str!("ui/main.xml"));

    let overlay: gtk4::Overlay = builder.object("overlay").unwrap();
    let whiteboard: gtk4::DrawingArea = builder.object("whiteboard").unwrap();
    let toolbox: gtk4::Box = builder.object("toolbox").unwrap();
    let toolbox_wrapper: gtk4::ListBox = builder.object("toolbox_wrapper").unwrap();

    overlay.add_overlay(&toolbox_wrapper);

    // Make the window background semi-transparent
    window.set_css_classes(&["main-window"]);

    let css = gtk4::CssProvider::new();
    css.load_from_data(include_str!("ui/custom.css"));
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().unwrap(),
        &css,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    window.set_content(Some(&overlay));
    window.present();

    // Return widgets
    (window, overlay, whiteboard, toolbox, toolbox_wrapper)
}
