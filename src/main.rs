mod draw;
mod ui;

use gtk4::prelude::*;
use libadwaita::Application;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Create a new Adw application.
    let app = libadwaita::Application::builder()
        .application_id("io.github.stellarst0rm.whiteboard")
        .build();

    app.connect_activate(app_closure);
    app.run();
}

fn app_closure(app: &Application) {
    // Build the UI
    let (window, _overlay, whiteboard, toolbox, _toolbox_wrapper) = ui::build(app);

    // Setup drawing
    let drawing_state = Rc::new(RefCell::new(draw::DrawingState {
        whiteboard: whiteboard.clone(),
        stroke_width: draw::SCROLL_MIN_CLAMP,
        stroke_color: gtk4::gdk::RGBA::new(1.0, 0.0, 0.0, 1.0),

        max_strokes: 1_000,
        strokes: Vec::new(),
    }));

    draw::setup(&window, &whiteboard, drawing_state.clone());

    // Make the UI buttons!
    let color_button = gtk4::Button::with_label("Change Color / Color Picker");
    let undo_button = gtk4::Button::with_label("Undo Last Stroke");
    let help_button = gtk4::Button::with_label("Help");

    let drawing_state_clone = drawing_state.clone();
    color_button.connect_clicked(move |_| {
        let dialog = gtk4::ColorChooserDialog::new(Some("Choose A Color"), Some(&window));
        dialog.set_rgba(&drawing_state_clone.borrow().stroke_color);
        dialog.set_show_editor(true);

        let dialog_clone = Rc::new(RefCell::new(dialog));
        let dialog_clone_closure = dialog_clone.clone();

        let drawing_state_clone = drawing_state_clone.clone();
        dialog_clone.borrow().run_async(move |obj, _| {
            obj.close();
            drawing_state_clone.borrow_mut().stroke_color = dialog_clone_closure.borrow().rgba();
        });
    });

    let drawing_state_clone = drawing_state.clone();
    undo_button.connect_clicked(move |_| {
        let mut state = drawing_state_clone.borrow_mut();
        state.strokes.pop();
        state.whiteboard.queue_draw();
    });

    help_button.connect_clicked(|_| {
        let builder = gtk4::Builder::from_string(include_str!("ui/help_dialog.xml"));

        let dialog: gtk4::AboutDialog = builder.object("dialog").unwrap();
        let action_container: gtk4::Box = builder.object("action").unwrap();
        let result_container: gtk4::Box = builder.object("result").unwrap();

        // Add data to containers
        const LABELS: [(&str, &str); 2] = [
            ("Control + Z:", "Undo Last Stroke"),
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

    toolbox.append(&color_button);
    toolbox.append(&undo_button);
    toolbox.append(&gtk4::Separator::new(gtk4::Orientation::Vertical));
    toolbox.append(&help_button);
}
