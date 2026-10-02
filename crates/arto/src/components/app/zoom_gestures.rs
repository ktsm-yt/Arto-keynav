use dioxus::desktop::window;
use dioxus::prelude::*;

use crate::config::{normalize_content_zoom, MAX_SIDEBAR_ZOOM, MIN_SIDEBAR_ZOOM};
use crate::state::AppState;

#[derive(serde::Deserialize)]
struct ZoomInput {
    ui: bool,
    kind: String,
    value: f64,
}

fn zoom_value(current: f64, start: f64, input: &ZoomInput) -> f64 {
    if !input.value.is_finite() {
        return current;
    }
    let value = match input.kind.as_str() {
        "scale" if input.value > 0.0 => start * input.value,
        "step" => current + input.value,
        "reset" => 1.0,
        _ => return current,
    };
    if input.ui {
        ((value * 100.0).round() / 100.0).clamp(MIN_SIDEBAR_ZOOM, MAX_SIDEBAR_ZOOM)
    } else {
        normalize_content_zoom(value)
    }
}

pub(super) fn setup_zoom_gestures(mut state: AppState) {
    // Whole-WebView zoom is per window; the existing document zoom stays independent.
    let mut ui_zoom = use_signal(|| 1.0);
    use_effect(move || window().set_zoom_level(ui_zoom()));
    use_hook(move || {
        spawn(async move {
            let mut eval = document::eval(indoc::indoc! {r#"
                let pinching = false;
                let ui = false;
                let optionHeld = false;
                const send = (kind, value, wholeUI = ui) => {
                    if (Number.isFinite(value)) dioxus.send({ ui: wholeUI, kind, value });
                };
                document.addEventListener('gesturestart', event => {
                    event.preventDefault();
                    pinching = true;
                    // macOS WebKit's native gesture events omit modifier flags.
                    ui = Boolean(event.altKey || optionHeld);
                    send('start', 1);
                }, { passive: false });
                document.addEventListener('gesturechange', event => {
                    if (!pinching) return;
                    event.preventDefault();
                    send('scale', event.scale);
                }, { passive: false });
                document.addEventListener('gestureend', event => {
                    event.preventDefault();
                    pinching = false;
                }, { passive: false });
                document.addEventListener('keydown', event => {
                    optionHeld = event.altKey;
                    if (event.isComposing || !event.metaKey || !event.shiftKey || event.altKey) return;
                    let kind = 'step';
                    let value;
                    if (event.code === 'Equal' || event.code === 'NumpadAdd') value = 0.05;
                    else if (event.code === 'Minus' || event.code === 'NumpadSubtract') value = -0.05;
                    else if (event.code === 'Digit0') { kind = 'reset'; value = 0; }
                    else return;
                    event.preventDefault();
                    event.stopImmediatePropagation();
                    send(kind, value, true);
                }, { capture: true });
                document.addEventListener('keyup', event => { optionHeld = event.altKey; }, { capture: true });
                window.addEventListener('blur', () => { optionHeld = false; });
            "#});
            let mut start = 1.0;
            while let Ok(input) = eval.recv::<ZoomInput>().await {
                let current = if input.ui {
                    ui_zoom()
                } else {
                    *state.zoom_level.peek()
                };
                if input.kind == "start" {
                    start = current;
                    continue;
                }
                let next = zoom_value(current, start, &input);
                if input.ui {
                    ui_zoom.set(next);
                } else {
                    state.zoom_level.set(next);
                }
            }
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinch_uses_gesture_start_and_keeps_ui_and_content_limits_separate() {
        let mut input = ZoomInput {
            ui: false,
            kind: "scale".into(),
            value: 1.2,
        };
        assert_eq!(zoom_value(1.5, 1.5, &input), 1.8);
        assert_eq!(zoom_value(1.8, 1.5, &input), 1.8);
        input.value = 10.0;
        assert_eq!(zoom_value(1.0, 1.0, &input), 5.0);
        input.ui = true;
        assert_eq!(zoom_value(1.0, 1.0, &input), 2.0);
        input.kind = "step".into();
        input.value = 0.05;
        assert_eq!(zoom_value(1.0, 1.0, &input), 1.05);
        input.value = -0.05;
        assert_eq!(zoom_value(1.05, 1.0, &input), 1.0);
        input.value = f64::NAN;
        assert_eq!(zoom_value(1.3, 1.0, &input), 1.3);
        input.kind = "reset".into();
        input.value = 0.0;
        assert_eq!(zoom_value(1.3, 1.0, &input), 1.0);
    }
}
