use crate::components::icon::{Icon, IconName};
use crate::config::Config;
use crate::utils::file_operations;
use dioxus::prelude::*;

#[component]
pub fn AboutTab() -> Element {
    let icon = crate::assets::app_icon_data_url();
    let version_text = format!("バージョン {}", env!("ARTO_BUILD_VERSION"));
    let config_dir_path = Config::path()
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    let config_dir_text = config_dir_path.display().to_string();

    rsx! {
        div {
            class: "about-page",

            div {
                class: "about-container",

                // Icon
                div {
                    class: "about-icon",
                    img {
                        src: "{icon}",
                        alt: "Arto Keynav",
                    }
                }

                // Title
                h2 { class: "about-title", "Arto Keynav" }

                // Version
                p { class: "about-version", "{version_text}" }

                // Tagline
                p { class: "about-tagline", "Markdownを読むためのアプリ。" }

                // Description
                p { class: "about-description",
                    "上下キーでファイルを切り替えられる、Artoの非公式派生版です。"
                }

                // Configuration directory
                div {
                    class: "about-config-dir",
                    p { class: "about-config-dir-label", "設定フォルダ" }
                    div {
                        class: "about-config-dir-row",
                        input {
                            class: "about-config-dir-input",
                            r#type: "text",
                            value: "{config_dir_text}",
                            readonly: true,
                        }
                        button {
                            class: "about-config-dir-button",
                            onclick: {
                                let config_dir_path = config_dir_path.clone();
                                move |_| {
                                    file_operations::open_directory_in_finder(&config_dir_path);
                                }
                            },
                            span { class: "about-link-icon", Icon { name: IconName::FolderOpen, size: 18 } }
                            span { class: "about-link-text", "Finderで開く" }
                        }
                    }
                }

                // Links (card style like no-file-hints)
                div {
                    class: "about-links",
                    a {
                        href: "https://github.com/ktsm-yt/Arto-keynav",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "about-link",
                        span { class: "about-link-icon", Icon { name: IconName::BrandGithub, size: 20 } }
                        span { class: "about-link-text", "GitHubで見る" }
                    }
                    a {
                        href: "https://github.com/ktsm-yt/Arto-keynav/issues",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "about-link",
                        span { class: "about-link-icon", Icon { name: IconName::Bug, size: 20 } }
                        span { class: "about-link-text", "問題を報告" }
                    }
                }

                // Footer
                div {
                    class: "about-footer",
                    p { "Arto by lambdalisue / Keynav fork by ktsm-yt" }
                    p { "Copyright © 2025 lambdalisue" }
                }
            }
        }
    }
}
