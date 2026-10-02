use super::super::form_controls::{OptionCardItem, OptionCards, ToggleRow};
use crate::config::Config;
use crate::markdown::{RawHtml, RenderOptions};
use dioxus::prelude::*;

/// What the renderer reads out of a document.
///
/// GFM — tables, task lists, strikethrough, footnotes — is not offered here.
/// Those are what a document written for GitHub contains, and a reader that
/// showed them as literal pipes and brackets would be broken rather than
/// configured. What is offered is the layer above that, where a construct's
/// syntax collides with prose somebody actually writes.
#[component]
pub fn MarkdownTab(config: Signal<Config>) -> Element {
    let markdown = config.read().markdown.clone();
    let defaults = RenderOptions::default();

    rsx! {
        div {
            class: "preferences-pane",

            h3 { class: "preference-section-title", "構文" }

            ToggleRow {
                label: "URLを自動でリンクにする".to_string(),
                description: Some("リンク記法の角括弧がない単独のURLもリンクにします。".to_string()),
                checked: markdown.auto_link_urls,
                on_change: move |on| config.write().markdown.auto_link_urls = on,
                shipped: Some(defaults.auto_link_urls),
            }

            ToggleRow {
                label: "数式".to_string(),
                description: Some("$…$ と $$…$$ を数式として表示します。".to_string()),
                checked: markdown.math,
                on_change: move |on| config.write().markdown.math = on,
                shipped: Some(defaults.math),
            }

            ToggleRow {
                label: "Wikiリンク".to_string(),
                description: Some("Obsidian形式の [[ページ]] と [[ページ|表示名]] をリンクとして開きます。".to_string()),
                checked: markdown.wiki_links,
                on_change: move |on| config.write().markdown.wiki_links = on,
                shipped: Some(defaults.wiki_links),
            }

            ToggleRow {
                label: "上付き文字".to_string(),
                description: Some("^text^ を上付き文字として表示します。".to_string()),
                checked: markdown.superscript,
                on_change: move |on| config.write().markdown.superscript = on,
                shipped: Some(defaults.superscript),
            }

            ToggleRow {
                label: "下付き文字".to_string(),
                description: Some("~text~ を下付き文字として表示します。".to_string()),
                checked: markdown.subscript,
                on_change: move |on| config.write().markdown.subscript = on,
                shipped: Some(defaults.subscript),
            }

            ToggleRow {
                label: "定義リスト".to_string(),
                description: Some("用語の次にコロンで始まる行を置く記法を定義リストとして表示します。".to_string()),
                checked: markdown.definition_lists,
                on_change: move |on| config.write().markdown.definition_lists = on,
                shipped: Some(defaults.definition_lists),
            }

            ToggleRow {
                label: "見出し属性".to_string(),
                description: Some("見出し末尾の {#id .class} を文字列として表示せず、属性として扱います。".to_string()),
                checked: markdown.heading_attributes,
                on_change: move |on| config.write().markdown.heading_attributes = on,
                shipped: Some(defaults.heading_attributes),
            }

            h3 { class: "preference-section-title", "文字組み" }

            ToggleRow {
                label: "スマート句読点".to_string(),
                description: Some("直線の引用符や --、... を組版用の記号に置き換えます。".to_string()),
                checked: markdown.smart_punctuation,
                on_change: move |on| config.write().markdown.smart_punctuation = on,
                shipped: Some(defaults.smart_punctuation),
            }

            ToggleRow {
                label: "日本語の句読点をまたぐ強調".to_string(),
                description: Some("**強調。** のように、日本語の句読点を含む強調を表示します。".to_string()),
                checked: markdown.cjk_emphasis,
                on_change: move |on| config.write().markdown.cjk_emphasis = on,
                shipped: Some(defaults.cjk_emphasis),
            }

            h3 { class: "preference-section-title", "見出し" }

            ToggleRow {
                label: "見出し横にパーマリンクを表示".to_string(),
                description: Some("GitHubと同様に、見出しへリンクする # を表示します。".to_string()),
                checked: markdown.heading_permalinks,
                on_change: move |on| config.write().markdown.heading_permalinks = on,
                shipped: Some(defaults.heading_permalinks),
            }

            h3 { class: "preference-section-title", "生のHTML" }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "Markdown内のHTML" }
                    p {
                        class: "preference-description",
                        "<kbd>、<details>、幅を指定した <img> など、Markdownで表せないHTMLの扱いを選びます。"
                    }
                }
                OptionCards {
                    name: "markdown-raw-html".to_string(),
                    options: vec![
                        OptionCardItem {
                            icon: None,
                            value: RawHtml::Allow,
                            title: "許可".to_string(),
                            description: Some("すべてのタグをそのまま表示".to_string()),
                        },
                        OptionCardItem {
                            icon: None,
                            value: RawHtml::Filter,
                            title: "フィルター".to_string(),
                            description: Some("危険なタグを除いて表示".to_string()),
                        },
                        OptionCardItem {
                            icon: None,
                            value: RawHtml::Escape,
                            title: "エスケープ".to_string(),
                            description: Some("HTMLを文字列として表示".to_string()),
                        },
                    ],
                    selected: markdown.raw_html,
                    on_change: move |raw_html| {
                        config.write().markdown.raw_html = raw_html;
                    },
                    shipped: Some(defaults.raw_html),
                }
            }
        }
    }
}
