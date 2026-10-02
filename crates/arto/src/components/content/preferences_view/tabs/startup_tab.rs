use super::super::form_controls::{ChoiceItem, ChoiceRow};
use crate::config::{Config, NewWindowBehavior, StartupBehavior};
use dioxus::prelude::*;

/// Whether a window opens with the defaults or carries something over.
///
/// This is one question — "the default, or what the last window had?" — asked
/// of every setting that has a default. It used to be asked twice at the foot
/// of every pane, which put eleven copies of the same two cards across the
/// preferences and left no way to see what a window would actually open with.
/// Asked once, in a column, the answer reads as the single decision it is.
#[component]
pub fn StartupTab(config: Signal<Config>) -> Element {
    let cfg = config.read().clone();
    let defaults = Config::default();

    rsx! {
        div {
            class: "preferences-pane",

            h3 { class: "preference-section-title", "Artoの起動時" }

            p {
                class: "preference-lede",
                "最初のウィンドウを既定値で開くか、最後に閉じたウィンドウの状態を引き継ぐかを選びます。"
            }

            OnStartupRow {
                name: "startup-theme",
                label: "テーマ",
                selected: cfg.theme.on_startup,
                on_change: move |value| config.write().theme.on_startup = value,
                shipped: Some(defaults.theme.on_startup),
            }
            OnStartupRow {
                name: "startup-window-size",
                label: "ウィンドウサイズ",
                selected: cfg.window_size.on_startup,
                on_change: move |value| config.write().window_size.on_startup = value,
                shipped: Some(defaults.window_size.on_startup),
            }
            OnStartupRow {
                name: "startup-window-position",
                label: "ウィンドウ位置",
                selected: cfg.window_position.on_startup,
                on_change: move |value| config.write().window_position.on_startup = value,
                shipped: Some(defaults.window_position.on_startup),
            }
            OnStartupRow {
                name: "startup-zoom",
                label: "拡大率",
                selected: cfg.zoom.on_startup,
                on_change: move |value| config.write().zoom.on_startup = value,
                shipped: Some(defaults.zoom.on_startup),
            }
            OnStartupRow {
                name: "startup-panel",
                label: "サイドバー",
                selected: cfg.sidebar.on_startup,
                on_change: move |value| config.write().sidebar.on_startup = value,
                shipped: Some(defaults.sidebar.on_startup),
            }
            OnStartupRow {
                name: "startup-folder",
                label: "フォルダ",
                selected: cfg.directory.on_startup,
                on_change: move |value| config.write().directory.on_startup = value,
                shipped: Some(defaults.directory.on_startup),
            }

            h3 { class: "preference-section-title", "新しいウィンドウを開くとき" }

            p {
                class: "preference-lede",
                "新しいウィンドウを既定値で開くか、開いた元のウィンドウの状態を引き継ぐかを選びます。"
            }

            OnNewWindowRow {
                name: "new-window-theme",
                label: "テーマ",
                selected: cfg.theme.on_new_window,
                on_change: move |value| config.write().theme.on_new_window = value,
                shipped: Some(defaults.theme.on_new_window),
            }
            OnNewWindowRow {
                name: "new-window-size",
                label: "ウィンドウサイズ",
                selected: cfg.window_size.on_new_window,
                on_change: move |value| config.write().window_size.on_new_window = value,
                shipped: Some(defaults.window_size.on_new_window),
            }
            OnNewWindowRow {
                name: "new-window-position",
                label: "ウィンドウ位置",
                selected: cfg.window_position.on_new_window,
                on_change: move |value| config.write().window_position.on_new_window = value,
                shipped: Some(defaults.window_position.on_new_window),
            }
            OnNewWindowRow {
                name: "new-window-zoom",
                label: "拡大率",
                selected: cfg.zoom.on_new_window,
                on_change: move |value| config.write().zoom.on_new_window = value,
                shipped: Some(defaults.zoom.on_new_window),
            }
            OnNewWindowRow {
                name: "new-window-panel",
                label: "サイドバー",
                selected: cfg.sidebar.on_new_window,
                on_change: move |value| config.write().sidebar.on_new_window = value,
                shipped: Some(defaults.sidebar.on_new_window),
            }
        }
    }
}

#[component]
fn OnStartupRow(
    name: &'static str,
    label: &'static str,
    selected: StartupBehavior,
    on_change: EventHandler<StartupBehavior>,
    shipped: Option<StartupBehavior>,
) -> Element {
    rsx! {
        ChoiceRow {
            name: name.to_string(),
            label: label.to_string(),
            options: vec![
                ChoiceItem {
                    value: StartupBehavior::Default,
                    label: "既定値".to_string(),
                },
                ChoiceItem {
                    value: StartupBehavior::LastClosed,
                    label: "最後に閉じた状態".to_string(),
                },
            ],
            selected,
            on_change: move |value| on_change.call(value),
            shipped,
        }
    }
}

#[component]
fn OnNewWindowRow(
    name: &'static str,
    label: &'static str,
    selected: NewWindowBehavior,
    on_change: EventHandler<NewWindowBehavior>,
    shipped: Option<NewWindowBehavior>,
) -> Element {
    rsx! {
        ChoiceRow {
            name: name.to_string(),
            label: label.to_string(),
            options: vec![
                ChoiceItem {
                    value: NewWindowBehavior::Default,
                    label: "既定値".to_string(),
                },
                ChoiceItem {
                    value: NewWindowBehavior::LastFocused,
                    label: "最後に操作した状態".to_string(),
                },
            ],
            selected,
            on_change: move |value| on_change.call(value),
            shipped,
        }
    }
}
