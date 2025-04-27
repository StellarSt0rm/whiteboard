use crate::consts::*;
use gtk4::prelude::*;

pub fn drag_begin_gesture(drawing_state: crate::AppState, gesture_drag: &gtk4::GestureDrag) {
    gesture_drag.connect_drag_begin(move |_, x, y| {
        let mut state = drawing_state.borrow_mut();
        let stroke_width = state.stroke_width.clone();
        let stroke_color = state.stroke_color.clone();

        state.drawing = true;
        state.strokes.push(super::Stroke {
            stroke_width,
            stroke_color,

            points: vec![(x, y)],
        });

        state.whiteboard.grab_focus(); // Fixes some annoyances
        state.whiteboard.queue_draw();
    });
}

pub fn drag_update_gesture(drawing_state: crate::AppState, gesture_drag: &gtk4::GestureDrag) {
    gesture_drag.connect_drag_update(move |gesture, x, y| {
        let mut state = drawing_state.borrow_mut();

        let start = gesture.start_point().unwrap();
        state
            .strokes
            .last_mut()
            .unwrap()
            .points
            .push((x + start.0, y + start.1));

        state.whiteboard.queue_draw();
    });
}

pub fn drag_end_gesture(drawing_state: crate::AppState, gesture_drag: &gtk4::GestureDrag) {
    gesture_drag.connect_drag_end(move |_, _, _| drawing_state.borrow_mut().drawing = false);
}

// Hide the dirty cloning behind a pretty function
pub fn set_drawing_function(drawing_state: crate::AppState) {
    let state = drawing_state.borrow();

    let drawing_state_clone = drawing_state.clone();
    state
        .whiteboard
        .set_draw_func(move |_, cr, _, _| drawing_function(drawing_state_clone.clone(), cr))
}

fn drawing_function(drawing_state: crate::AppState, cr: &gtk4::cairo::Context) {
    let mut state = drawing_state.borrow_mut();

    let len = match state.strokes.len().cmp(&MAX_STROKES) {
        std::cmp::Ordering::Greater => state.strokes.len() - MAX_STROKES,
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
                stroke.stroke_width / 2.0,
                0.0,
                std::f64::consts::TAU,
            );
            cr.fill().unwrap();

            continue;
        }

        // Make the line from the stroke points
        cr.move_to(
            stroke.points.first().unwrap().0,
            stroke.points.first().unwrap().1,
        );

        for &(x, y) in &stroke.points[1..] {
            cr.line_to(x, y);
        }
        cr.stroke().unwrap();
    }
}
