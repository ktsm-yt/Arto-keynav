use super::super::form_controls::{OptionCardItem, OptionCards, SliderInput, ToggleRow};
use crate::config::{
    normalize_sidebar_zoom, Config, OpenFromPanel, MAX_SIDEBAR_ZOOM, MIN_SIDEBAR_ZOOM, ZOOM_STEP,
};
use crate::events::SET_SIDEBAR_ZOOM_IN_WINDOW;
use dioxus::desktop::tao::window::WindowId;
use dioxus::prelude::*;

/// The panel beside the document: the file tree and the history.
///
/// The width the document keeps against it is a reading choice rather than a
/// panel one, so it is set in [`super::reading_tab`].
#[component]
pub fn PanelTab(
    config: Signal<Config>,
    /// The window the "Current Settings" section acts on.
    window_id: WindowId,
    current_width: f64,
    /// That window's panel zoom, which this pane both shows and sets.
    mut current_zoom: Signal<f64>,
) -> Element {
    let sidebar_cfg = config.read().sidebar.clone();
    let defaults = Config::default();

    rsx! {
        div {
            class: "preferences-pane",

            h3 { class: "preference-section-title", "現在の設定" }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "現在の拡大率" }
                    p { class: "preference-description", "現在のウィンドウのサイドバー拡大率です。" }
                }
                SliderInput {
                    value: current_zoom(),
                    min: MIN_SIDEBAR_ZOOM,
                    max: MAX_SIDEBAR_ZOOM,
                    step: ZOOM_STEP,
                    unit: "x".to_string(),
                    decimals: 1,
                    on_change: move |new_zoom| {
                        let normalized = normalize_sidebar_zoom(new_zoom);
                        current_zoom.set(normalized);
                        let _ = SET_SIDEBAR_ZOOM_IN_WINDOW.send((window_id, normalized));
                    },
                    default_value: Some(sidebar_cfg.default_zoom_level),
                }
            }

            h3 { class: "preference-section-title", "既定の設定" }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "既定の幅" }
                    p { class: "preference-description", "ウィンドウを開くときのサイドバー幅です。" }
                }
                SliderInput {
                    value: sidebar_cfg.default_width,
                    min: 200.0,
                    max: 600.0,
                    step: 10.0,
                    unit: "px".to_string(),
                    on_change: move |new_width| {
                        config.write().sidebar.default_width = new_width;
                    },
                    current_value: Some(current_width),
                    shipped: Some(defaults.sidebar.default_width),
                }
            }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "既定の拡大率" }
                    p { class: "preference-description", "ウィンドウを開くときのサイドバー内容の拡大率です。" }
                }
                SliderInput {
                    value: sidebar_cfg.default_zoom_level,
                    min: MIN_SIDEBAR_ZOOM,
                    max: MAX_SIDEBAR_ZOOM,
                    step: ZOOM_STEP,
                    unit: "x".to_string(),
                    decimals: 1,
                    on_change: move |new_zoom| {
                        config.write().sidebar.default_zoom_level = new_zoom;
                    },
                    current_value: Some(current_zoom()),
                    shipped: Some(defaults.sidebar.default_zoom_level),
                }
            }

            ToggleRow {
                label: "既定で固定する".to_string(),
                description: Some("固定するとサイドバーが本文の横に表示されます。解除するとホバー時に本文の上へ表示されます。".to_string()),
                checked: sidebar_cfg.default_pinned,
                on_change: move |on| config.write().sidebar.default_pinned = on,
                shipped: Some(defaults.sidebar.default_pinned),
            }

            ToggleRow {
                label: "すべてのファイルを表示".to_string(),
                description: Some("オフにすると、ツリーにはMarkdownファイルだけを表示します。".to_string()),
                checked: sidebar_cfg.default_show_all_files,
                on_change: move |on| config.write().sidebar.default_show_all_files = on,
                shipped: Some(defaults.sidebar.default_show_all_files),
            }

            h3 { class: "preference-section-title", "動作" }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "ドキュメントを開いた後" }
                    p {
                        class: "preference-description",
                        "サイドバーの項目からドキュメントを開いた後の表示を選びます。"
                    }
                }
                OptionCards {
                    name: "panel-on-open".to_string(),
                    options: vec![
                        OptionCardItem {
                            icon: None,
                            value: OpenFromPanel::KeepOpen,
                            title: "サイドバーを開いたままにする".to_string(),
                            description: Some("一覧をそのまま表示します".to_string()),
                        },
                        OptionCardItem {
                            icon: None,
                            value: OpenFromPanel::ClosePanel,
                            title: "サイドバーを閉じる".to_string(),
                            description: Some("本文だけを表示します".to_string()),
                        },
                    ],
                    selected: sidebar_cfg.on_open,
                    on_change: move |new_behavior| {
                        config.write().sidebar.on_open = new_behavior;
                    },
                    shipped: Some(defaults.sidebar.on_open),
                }
            }
        }
    }
}
