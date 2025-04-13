use crate::consts::*;
use gtk4::prelude::*;

pub fn scroll_shortcut(
    drawing_state: super::AppState,
    whiteboard: &libadwaita::ApplicationWindow,
    stroke_size_button: gtk4::SpinButton,
) {
    let scroll_controller =
        gtk4::EventControllerScroll::new(gtk4::EventControllerScrollFlags::VERTICAL);
    scroll_controller.connect_scroll(move |_, _, dy| {
        let mut state = drawing_state.borrow_mut();

        state.stroke_width =
            (state.stroke_width - dy * SCROLL_MULTIPLY).clamp(SCROLL_MIN_CLAMP, SCROLL_MAX_CLAMP);

        // Causes error because the SpinButton then tries to re-borrow,
        // that's why we need to drop the state before calling set_value()
        let stroke_width = state.stroke_width.clone();
        drop(state);
        stroke_size_button.set_value(stroke_width);

        true.into()
    });

    whiteboard.add_controller(scroll_controller);
}

pub fn control_z_shortcut(
    drawing_state: super::AppState,
    shortcut_controller: &gtk4::ShortcutController,
) {
    let callback = gtk4::CallbackAction::new(move |_, _| {
        let mut state = drawing_state.borrow_mut();

        state.strokes.pop();
        state.whiteboard.queue_draw();

        true.into()
    });

    shortcut_controller.add_shortcut(gtk4::Shortcut::new(
        gtk4::ShortcutTrigger::parse_string("<Control>z"),
        Some(callback),
    ));
}

pub fn control_c_shortcut(
    drawing_state: super::AppState,
    shortcut_controller: &gtk4::ShortcutController,
) {
    let callback = gtk4::CallbackAction::new(move |_, _| {
        let mut state = drawing_state.borrow_mut();

        state.strokes.drain(..);
        state.whiteboard.queue_draw();

        true.into()
    });

    shortcut_controller.add_shortcut(gtk4::Shortcut::new(
        gtk4::ShortcutTrigger::parse_string("<Control>c"),
        Some(callback),
    ));
}
