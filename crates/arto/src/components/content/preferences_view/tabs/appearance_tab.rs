use super::super::form_controls::{OptionCardItem, OptionCards, ThemePicker};
use crate::components::icon::IconName;
use crate::config::Config;
use crate::theme::{preview_theme, Theme};
use dioxus::prelude::*;

/// Which of GitHub's themes the window paints.
///
/// What the window does with the theme on the next startup, or in the next
/// window, is asked once for everything in [`super::startup_tab`] rather than
/// again here.
#[component]
pub fn AppearanceTab(config: Signal<Config>) -> Element {
    let theme = config.read().theme.clone();
    let defaults = Config::default().theme;

    rsx! {
        div {
            class: "preferences-pane",

            h3 { class: "preference-section-title", "モード" }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "既定のテーマ" }
                    p { class: "preference-description", "ライト、ダーク、またはシステム設定に合わせたテーマを選びます。" }
                }
                OptionCards {
                    name: "theme-default".to_string(),
                    options: vec![
                        OptionCardItem {
                            value: Theme::Auto,
                            icon: Some(IconName::SunMoon),
                            title: "自動".to_string(),
                            description: None,
                        },
                        OptionCardItem {
                            value: Theme::Light,
                            icon: Some(IconName::Sun),
                            title: "ライト".to_string(),
                            description: None,
                        },
                        OptionCardItem {
                            value: Theme::Dark,
                            icon: Some(IconName::Moon),
                            title: "ダーク".to_string(),
                            description: None,
                        },
                    ],
                    selected: theme.default_theme,
                    on_change: move |new_theme| {
                        config.write().theme.default_theme = new_theme;
                    },
                    shipped: Some(defaults.default_theme),
                }
            }

            h3 { class: "preference-section-title", "テーマ" }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "ライトテーマ" }
                    p { class: "preference-description", "ライトモードで使うGitHubテーマを選びます。" }
                }
                ThemePicker {
                    name: "theme-light".to_string(),
                    dark_mode: false,
                    selected: theme.light_theme,
                    on_change: move |new_theme| {
                        config.write().theme.light_theme = new_theme;
                        preview_theme(new_theme);
                    },
                    shipped: Some(defaults.light_theme),
                }
            }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "ダークテーマ" }
                    p { class: "preference-description", "ダークモードで使うGitHubテーマを選びます。ライトテーマも選択できます。" }
                }
                ThemePicker {
                    name: "theme-dark".to_string(),
                    dark_mode: true,
                    selected: theme.dark_theme,
                    on_change: move |new_theme| {
                        config.write().theme.dark_theme = new_theme;
                        preview_theme(new_theme);
                    },
                    shipped: Some(defaults.dark_theme),
                }
            }
        }
    }
}
