mod draw;
mod shortcuts;
mod ui;

use gtk4::prelude::*;
use libadwaita::Application;
use std::cell::RefCell;
use std::rc::Rc;

// Structs
mod consts {
    pub const SCROLL_MAX_CLAMP: f64 = 100.0;
    pub const SCROLL_MIN_CLAMP: f64 = 5.0;
    pub const SCROLL_MULTIPLY: f64 = 5.0; // How much to multiply `dy` by
}

type AppState = Rc<RefCell<DrawingState>>;

pub struct DrawingState {
    pub whiteboard: gtk4::DrawingArea,
    pub stroke_width: f64,
    pub stroke_color: gtk4::gdk::RGBA,

    pub max_strokes: usize,
    pub strokes: Vec<Stroke>,
}

#[derive(Debug)]
pub struct Stroke {
    pub stroke_width: f64,
    pub stroke_color: gtk4::gdk::RGBA,

    pub points: Vec<(f64, f64)>,
}
// ---

fn main() {
    // Create a new Adw application.
    let app = libadwaita::Application::builder()
        .application_id("io.github.stellarst0rm.whiteboard")
        .build();

    app.connect_activate(app_closure);
    app.run();
}

fn app_closure(app: &Application) {
    let (window, _overlay, whiteboard, toolbox, _toolbox_wrapper) = ui::build(app);

    // Setup drawing
    let gesture_drag = gtk4::GestureDrag::new();
    let drawing_state = Rc::new(RefCell::new(DrawingState {
        whiteboard: whiteboard.clone(),
        stroke_width: consts::SCROLL_MIN_CLAMP,
        stroke_color: gtk4::gdk::RGBA::new(1.0, 0.0, 0.0, 1.0),

        max_strokes: 1_000,
        strokes: Vec::new(),
    }));

    draw::drag_begin_gesture(drawing_state.clone(), &gesture_drag);
    draw::drag_update_gesture(drawing_state.clone(), &gesture_drag);
    draw::set_drawing_function(drawing_state.clone());

    whiteboard.add_controller(gesture_drag);

    // Toolbox buttons
    ui::change_color_button(drawing_state.clone(), &toolbox, window.clone());
    let stroke_size_button = ui::stroke_size_button(drawing_state.clone(), &toolbox);

    ui::separator(&toolbox);
    ui::undo_stroke_button(drawing_state.clone(), &toolbox);
    ui::clear_screen_button(drawing_state.clone(), &toolbox);

    ui::separator(&toolbox);
    ui::help_button(&toolbox);
    ui::quit_button(&toolbox, window.clone());

    // Keyboard shortcuts
    let shortcut_controller = gtk4::ShortcutController::new();

    shortcuts::scroll_shortcut(drawing_state.clone(), &window, stroke_size_button);
    shortcuts::control_z_shortcut(drawing_state.clone(), &shortcut_controller);
    shortcuts::control_c_shortcut(drawing_state.clone(), &shortcut_controller);

    window.add_controller(shortcut_controller);
}
