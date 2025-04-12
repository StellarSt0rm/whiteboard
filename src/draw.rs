use gtk4::gdk::RGBA;
use gtk4::prelude::*;
use gtk4::{DrawingArea, GestureDrag};
use std::cell::RefCell;
use std::rc::Rc;

pub const SCROLL_MAX_CLAMP: f64 = 100.0;
pub const SCROLL_MIN_CLAMP: f64 = 5.0;
pub const SCROLL_MULTIPLY: f64 = 5.0; // How much to multiply dy

pub struct DrawingState {
    pub whiteboard: gtk4::DrawingArea,
    pub stroke_width: f64,
    pub stroke_color: RGBA,

    pub max_strokes: usize,
    pub strokes: Vec<Stroke>,
}

#[derive(Debug)]
pub struct Stroke {
    stroke_width: f64,
    stroke_color: RGBA,

    points: Vec<(f64, f64)>,
}

pub fn setup(
    window: &libadwaita::ApplicationWindow,
    whiteboard: &DrawingArea,
    drawing_state: Rc<RefCell<DrawingState>>,
) {
    // // // // // // // //
    // Gesture  Handlers //
    // // // // // // // //
    let gesture_drag = GestureDrag::new();

    let drawing_state_clone = drawing_state.clone();
    gesture_drag.connect_drag_begin(move |_, x, y| {
        let mut state = drawing_state_clone.borrow_mut();
        let stroke_width = state.stroke_width.clone();
        let stroke_color = state.stroke_color.clone();

        state.strokes.push(Stroke {
            stroke_width,
            stroke_color,

            points: vec![(x, y)],
        });
        state.whiteboard.queue_draw();
    });

    let drawing_state_clone = drawing_state.clone();
    gesture_drag.connect_drag_update(move |gesture, x, y| {
        let mut state = drawing_state_clone.borrow_mut();

        let start = gesture.start_point().unwrap();
        state
            .strokes
            .last_mut()
            .unwrap()
            .points
            .push((x + start.0, y + start.1));

        state.whiteboard.queue_draw();
    });

    whiteboard.add_controller(gesture_drag);

    // // // // // // // // //
    // Keyboard   Shortcuts //
    // // // // // // // // //
    let shortcut_controller = gtk4::ShortcutController::new();

    let drawing_state_clone = drawing_state.clone();
    shortcut_controller.add_shortcut({
        gtk4::Shortcut::new(
            gtk4::ShortcutTrigger::parse_string("<Control>z"),
            Some(gtk4::CallbackAction::new(move |_, _| {
                let mut state = drawing_state_clone.borrow_mut();

                state.strokes.pop();
                state.whiteboard.queue_draw();

                true.into()
            })),
        )
    });

    window.add_controller(shortcut_controller);

    // // // // // // // //
    // Other Controllers //
    // // // // // // // //
    let drawing_state_clone = drawing_state.clone();
    let scroll_controller =
        gtk4::EventControllerScroll::new(gtk4::EventControllerScrollFlags::VERTICAL);
    scroll_controller.connect_scroll(move |_, _, dy| {
        let mut state = drawing_state_clone.borrow_mut();

        state.stroke_width =
            (state.stroke_width - dy * SCROLL_MULTIPLY).clamp(SCROLL_MIN_CLAMP, SCROLL_MAX_CLAMP);

        true.into()
    });

    whiteboard.add_controller(scroll_controller);

    // // // // // // // //
    // Drawing  Function //
    // // // // // // // //
    whiteboard.set_draw_func(move |_, cr, _width, _height| {
        let mut state = drawing_state.borrow_mut();

        let len = match state.strokes.len().cmp(&state.max_strokes) {
            std::cmp::Ordering::Greater => state.strokes.len() - state.max_strokes,
            _ => 0,
        };

        state.strokes.drain(..len);

        for stroke in &state.strokes {
            // Make da line!
            cr.set_line_cap(gtk4::cairo::LineCap::Round);
            cr.set_line_join(gtk4::cairo::LineJoin::Round);

            cr.set_line_width(stroke.stroke_width);
            cr.set_source_color(&stroke.stroke_color);
            
            // Add one ball if the stroke lenght is one
            if stroke.points.len() == 1 {
                cr.arc(
                    stroke.points.last().unwrap().0, // X
                    stroke.points.last().unwrap().1, // Y
                    state.stroke_width / 2.0,
                    0.0,
                    std::f64::consts::TAU,
                );
                cr.fill().unwrap();

                continue;
            }

            // Make the line from the stroke points
            cr.move_to(
                stroke.points.first().unwrap().0, // X
                stroke.points.first().unwrap().1, // Y
            );

            for &(x, y) in &stroke.points[1..] {
                cr.line_to(x, y);
            }
            cr.stroke().unwrap();
        }
    });
}
