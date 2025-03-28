use gtk4::prelude::*;
use gtk4::{DrawingArea, GestureDrag};
use std::cell::RefCell;
use std::rc::Rc;

const SMOOTHNESS: f64 = 5.0;
const MAX_POINTS: usize = 1_000;

struct DrawingState {
    whiteboard: gtk4::DrawingArea,

    points: Vec<(f64, f64)>,
    last_point: Option<(f64, f64)>, // Also used to know that we're actively drawing
}

pub fn setup_draw(whiteboard: &DrawingArea) {
    let drawing_state = Rc::new(RefCell::new(DrawingState {
        whiteboard: whiteboard.clone(),

        points: Vec::new(),
        last_point: None,
    }));

    // Handle painting with a drag gesture
    let gesture_drag = GestureDrag::new();

    let drawing_state_clone = drawing_state.clone();
    gesture_drag.connect_drag_begin(move |_, x, y| {
        let mut state = drawing_state_clone.borrow_mut();

        state.points.push((x, y));
        if let None = state.last_point {
            state.last_point = Some((x, y));
        }

        state.whiteboard.queue_draw();
    });

    let drawing_state_clone = drawing_state.clone();
    gesture_drag.connect_drag_end(move |_, _, _| {
        let mut state = drawing_state_clone.borrow_mut();
        state.last_point = None;
    });

    let drawing_state_clone = drawing_state.clone();
    gesture_drag.connect_drag_update(move |gesture, x, y| {
        let mut state = drawing_state_clone.borrow_mut();
        if state.last_point.is_none() {
            return;
        }

        let start = gesture.start_point().unwrap();
        let new_pos = (x + start.0, y + start.1);
        let last_pos = state.last_point.unwrap();

        // Smoothness
        let dx = new_pos.0 - last_pos.0;
        let dy = new_pos.1 - last_pos.1;
        let steps = ((dx * dx + dy * dy).sqrt() / SMOOTHNESS).ceil() as usize;

        for i in 1..=steps {
            let t = i as f64 / steps as f64;

            let x = last_pos.0 + t * dx;
            let y = last_pos.1 + t * dy;

            state.points.push((x, y));
        }

        println!("{}", steps);

        state.last_point = Some(new_pos);
        state.whiteboard.queue_draw();
    });

    whiteboard.add_controller(gesture_drag);

    // Drawing function
    whiteboard.set_draw_func(move |_, cr, _width, _height| {
        let mut state = drawing_state.borrow_mut();

        let len = {
            if state.points.len() >= MAX_POINTS {
                state.points.len() - MAX_POINTS
            } else {
                0
            }
        };
        state.points = state.points[len..].to_vec();

        for &(x, y) in &state.points {
            cr.arc(x, y, 5.0, 0.0, std::f64::consts::TAU);
            cr.fill().ok();
        }
    });
}
