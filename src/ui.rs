use super::consts::*;

use gtk4::prelude::*;
use libadwaita::ApplicationWindow;
use libadwaita::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn build(
    app: &libadwaita::Application,
) -> (
    ApplicationWindow,
    gtk4::Overlay,
    gtk4::DrawingArea,
    gtk4::Box,
    gtk4::ListBox,
) {
    let window = ApplicationWindow::builder()
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

// // // // // // // //
// Toolbox   Buttons //
// // // // // // // //
pub fn separator(toolbox: &gtk4::Box) {
    toolbox.append(&gtk4::Separator::new(gtk4::Orientation::Vertical));
}

pub fn change_color_button(
    drawing_state: crate::AppState,
    toolbox: &gtk4::Box,
    window: ApplicationWindow,
) {
    let color_button = gtk4::Button::with_label("Change Color / Color Picker");

    color_button.connect_clicked(move |_| {
        let dialog = gtk4::ColorChooserDialog::new(Some("Choose A Color"), Some(&window));
        dialog.set_rgba(&drawing_state.borrow().stroke_color);
        dialog.set_show_editor(true);

        let dialog_clone = Rc::new(RefCell::new(dialog));
        let dialog_clone_closure = dialog_clone.clone();

        let drawing_state_clone = drawing_state.clone();
        dialog_clone.borrow().run_async(move |obj, _| {
            obj.close();
            drawing_state_clone.borrow_mut().stroke_color = dialog_clone_closure.borrow().rgba();
        });
    });

    toolbox.append(&color_button);
}

// Returns the spin button to then feed it into the Scroll Shortcut
pub fn stroke_size_button(drawing_state: crate::AppState, toolbox: &gtk4::Box) -> gtk4::SpinButton {
    let adjustment = gtk4::Adjustment::new(
        drawing_state.borrow().stroke_width,
        SCROLL_MIN_CLAMP,
        SCROLL_MAX_CLAMP,
        SCROLL_MULTIPLY,
        SCROLL_MULTIPLY,
        0.0,
    );

    let stroke_size_button = gtk4::SpinButton::builder()
        .adjustment(&adjustment)
        .numeric(true)
        .build();

    let drawing_state_clone = drawing_state.clone();
    stroke_size_button.connect_value_changed(move |obj| {
        drawing_state_clone.borrow_mut().stroke_width = obj.value();
    });

    toolbox.append(&stroke_size_button);
    stroke_size_button
}

pub fn undo_stroke_button(drawing_state: crate::AppState, toolbox: &gtk4::Box) {
    let undo_stroke_button = gtk4::Button::with_label("Undo Last Stroke");

    undo_stroke_button.connect_clicked(move |_| {
        let mut state = drawing_state.borrow_mut();
        state.strokes.pop();
        state.whiteboard.queue_draw();
    });

    toolbox.append(&undo_stroke_button)
}

pub fn clear_screen_button(drawing_state: crate::AppState, toolbox: &gtk4::Box) {
    let clear_button = gtk4::Button::with_label("Clear Screen");

    clear_button.connect_clicked(move |_| {
        let mut state = drawing_state.borrow_mut();
        state.strokes.drain(..);
        state.whiteboard.queue_draw();
    });

    toolbox.append(&clear_button)
}

pub fn help_button(toolbox: &gtk4::Box) {
    let help_button = gtk4::Button::with_label("Help");

    help_button.connect_clicked(|_| {
        let builder = gtk4::Builder::from_string(include_str!("ui/help_dialog.xml"));

        let dialog: gtk4::AboutDialog = builder.object("dialog").unwrap();
        let action_container: gtk4::Box = builder.object("action").unwrap();
        let result_container: gtk4::Box = builder.object("result").unwrap();

        // Add data to containers
        const LABELS: [(&str, &str); 3] = [
            ("Control + Z:", "Undo Last Stroke"),
            ("Control + C:", "Clear Screen"),
            ("Scroll Wheel:", "Change Stroke Size"),
        ];

        for (action, result) in LABELS {
            let action_label = gtk4::Label::new(Some(action));
            let result_label = gtk4::Label::new(Some(result));

            action_label.add_css_class("help-title");
            action_label.set_halign(gtk4::Align::End);
            result_label.set_halign(gtk4::Align::Start);

            action_container.append(&action_label);
            result_container.append(&result_label);
        }

        dialog.show();
    });

    toolbox.append(&help_button);
}
