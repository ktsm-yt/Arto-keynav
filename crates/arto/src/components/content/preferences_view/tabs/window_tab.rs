use super::super::form_controls::{
    DimensionInput, DirectoryPicker, OptionCardItem, OptionCards, ResetLine,
};
use crate::components::icon::IconName;
use crate::config::{
    Config, FileOpenBehavior, WindowDimension, WindowDimensionUnit, WindowPosition,
    WindowPositionMode, WindowSize,
};
use dioxus::prelude::*;
use dioxus_desktop::window;
use std::path::PathBuf;

/// Windows: how large they open, where they land, and which one a document
/// arrives in.
///
/// The startup folder is here for the same reason the routing is: both answer
/// what a window is holding when it appears, not what the file tree contains.
/// The places the reader keeps are not a setting at all — they are kept by
/// starring a folder, and every window already starts with all of them.
#[component]
pub fn WindowTab(config: Signal<Config>, current_directory: Option<PathBuf>) -> Element {
    let size_cfg = config.read().window_size.clone();
    let position_cfg = config.read().window_position.clone();
    let file_open = config.read().file_open;
    let default_directory = config.read().directory.default_directory.clone();
    let defaults = Config::default();

    let use_current_size = move |_| {
        let metrics = crate::window::metrics::capture_window_metrics(&window().window);
        config.write().window_size.default_size = WindowSize {
            width: WindowDimension {
                value: metrics.size.width as f64,
                unit: WindowDimensionUnit::Pixels,
            },
            height: WindowDimension {
                value: metrics.size.height as f64,
                unit: WindowDimensionUnit::Pixels,
            },
        };
    };

    let use_current_position = move |_| {
        let metrics = crate::window::metrics::capture_window_metrics(&window().window);
        let mut cfg = config.write();
        cfg.window_position.default_position = WindowPosition {
            x: WindowDimension {
                value: metrics.position.x as f64,
                unit: WindowDimensionUnit::Pixels,
            },
            y: WindowDimension {
                value: metrics.position.y as f64,
                unit: WindowDimensionUnit::Pixels,
            },
        };
        cfg.window_position.default_position_mode = WindowPositionMode::Coordinates;
    };

    rsx! {
        div {
            class: "preferences-pane",

            h3 { class: "preference-section-title", "サイズ" }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "既定のサイズ" }
                    p { class: "preference-description", "ウィンドウを開くときのサイズです。百分率は現在の画面サイズを基準にします。" }
                }
                div {
                    class: "dimension-row",
                    div {
                        class: "dimension-grid",
                        div {
                            class: "dimension-field",
                            label { "幅" }
                            DimensionInput {
                                value: size_cfg.default_size.width,
                                min: 0.0,
                                step: 1.0,
                                allow_negative_pixels: false,
                                on_change: move |new_value| {
                                    config.write().window_size.default_size.width = new_value;
                                },
                                shipped: Some(defaults.window_size.default_size.width),
                            }
                        }
                        div {
                            class: "dimension-field",
                            label { "高さ" }
                            DimensionInput {
                                value: size_cfg.default_size.height,
                                min: 0.0,
                                step: 1.0,
                                allow_negative_pixels: false,
                                on_change: move |new_value| {
                                    config.write().window_size.default_size.height = new_value;
                                },
                                shipped: Some(defaults.window_size.default_size.height),
                            }
                        }
                    }
                    button {
                        class: "use-current-button",
                        onclick: use_current_size,
                        "現在のサイズを使う"
                    }
                }
            }

            h3 { class: "preference-section-title", "位置" }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "既定の位置" }
                    p { class: "preference-description", "ウィンドウを開く位置です。百分率は利用可能な画面範囲を基準にします。" }
                }
                OptionCards {
                    name: "window-position-mode".to_string(),
                    options: vec![
                        OptionCardItem {
                            value: WindowPositionMode::Coordinates,
                            icon: Some(IconName::Command),
                            title: "座標指定".to_string(),
                            description: Some("下のX/Y値を使います".to_string()),
                        },
                        OptionCardItem {
                            value: WindowPositionMode::Mouse,
                            icon: Some(IconName::Click),
                            title: "マウス位置".to_string(),
                            description: Some("現在のマウス位置に開きます".to_string()),
                        },
                    ],
                    selected: position_cfg.default_position_mode,
                    on_change: move |new_mode| {
                        config.write().window_position.default_position_mode = new_mode;
                    },
                    shipped: Some(defaults.window_position.default_position_mode),
                }
                div {
                    class: "dimension-row spacing-top-sm",
                    div {
                        class: "dimension-grid",
                        div {
                            class: "dimension-field",
                            label { "X" }
                            DimensionInput {
                                value: position_cfg.default_position.x,
                                min: 0.0,
                                step: 1.0,
                                allow_negative_pixels: true,
                                on_change: move |new_value| {
                                    config.write().window_position.default_position.x = new_value;
                                },
                                shipped: Some(defaults.window_position.default_position.x),
                            }
                        }
                        div {
                            class: "dimension-field",
                            label { "Y" }
                            DimensionInput {
                                value: position_cfg.default_position.y,
                                min: 0.0,
                                step: 1.0,
                                allow_negative_pixels: true,
                                on_change: move |new_value| {
                                    config.write().window_position.default_position.y = new_value;
                                },
                                shipped: Some(defaults.window_position.default_position.y),
                            }
                        }
                    }
                    button {
                        class: "use-current-button",
                        onclick: use_current_position,
                        "現在の位置を使う"
                    }
                }
            }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "位置のずらし量" }
                    p { class: "preference-description", "近い位置に別のウィンドウがあるとき、新しいウィンドウをこの量だけずらします。" }
                }
                div {
                    class: "dimension-grid",
                    div {
                        class: "dimension-field",
                        label { "X" }
                        div {
                            class: "dimension-input",
                            input {
                                r#type: "number",
                                inputmode: "decimal",
                                min: "0",
                                step: "1",
                                value: "{position_cfg.position_offset.x}",
                                oninput: move |evt| {
                                    let fallback = config.read().window_position.position_offset.x;
                                    let value = evt.value().parse::<i32>().unwrap_or(fallback);
                                    config.write().window_position.position_offset.x = value.max(0);
                                },
                            }
                            span { class: "dimension-unit", "px" }
                        }
                    }
                    div {
                        class: "dimension-field",
                        label { "Y" }
                        div {
                            class: "dimension-input",
                            input {
                                r#type: "number",
                                inputmode: "decimal",
                                min: "0",
                                step: "1",
                                value: "{position_cfg.position_offset.y}",
                                oninput: move |evt| {
                                    let fallback = config.read().window_position.position_offset.y;
                                    let value = evt.value().parse::<i32>().unwrap_or(fallback);
                                    config.write().window_position.position_offset.y = value.max(0);
                                },
                            }
                            span { class: "dimension-unit", "px" }
                        }
                    }
                }
                // Two raw fields rather than a shared control, so the way back
                // is stated here instead of by the control.
                ResetLine {
                    shipped: (position_cfg.position_offset != defaults.window_position.position_offset)
                        .then(|| {
                            format!(
                                "{} × {}px",
                                defaults.window_position.position_offset.x,
                                defaults.window_position.position_offset.y,
                            )
                        }),
                    on_reset: move |_| {
                        config.write().window_position.position_offset =
                            Config::default().window_position.position_offset;
                    },
                }
            }

            h3 { class: "preference-section-title", "開くとき" }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "ファイルを開く場所" }
                    p {
                        class: "preference-description",
                        "Finder、コマンドライン、別のArtoから開いたファイルやフォルダを表示するウィンドウを選びます。"
                    }
                }
                OptionCards {
                    name: "window-file-open".to_string(),
                    options: vec![
                        OptionCardItem {
                            icon: None,
                            value: FileOpenBehavior::NewWindow,
                            title: "新しいウィンドウ".to_string(),
                            description: Some("常に新しいウィンドウで開きます".to_string()),
                        },
                        OptionCardItem {
                            icon: None,
                            value: FileOpenBehavior::LastFocused,
                            title: "最後に操作したウィンドウ".to_string(),
                            description: Some("最後に操作した表示中のウィンドウで開きます".to_string()),
                        },
                        OptionCardItem {
                            icon: None,
                            value: FileOpenBehavior::CurrentScreen,
                            title: "現在の画面".to_string(),
                            description: Some("カーソルがある画面の表示中ウィンドウで開きます".to_string()),
                        },
                    ],
                    selected: file_open,
                    on_change: move |new_behavior| {
                        config.write().file_open = new_behavior;
                    },
                    shipped: Some(defaults.file_open),
                }
            }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "起動時のフォルダ" }
                    p {
                        class: "preference-description",
                        "最初のウィンドウで開くフォルダです。空欄なら登録した場所だけで開始します。"
                    }
                }
                DirectoryPicker {
                    value: default_directory,
                    placeholder: "フォルダなし — 登録した場所だけで開始".to_string(),
                    current_directory: current_directory.clone(),
                    on_change: move |new_directory| {
                        config.write().directory.default_directory = new_directory;
                    },
                    shipped: Some(defaults.directory.default_directory.clone()),
                }
            }
        }
    }
}
