use super::super::form_controls::{
    ChoiceItem, ChoiceRow, OptionCardItem, OptionCards, SliderInput, ToggleRow,
};
use crate::config::{
    normalize_content_zoom, normalize_font_size, normalize_line_height, normalize_measure,
    CjkFontLanguage, Config, FontFamilyChoice, RecentTrace, LINE_HEIGHT_STEP, MAX_CONTENT_ZOOM,
    MAX_FONT_SIZE, MAX_LINE_HEIGHT, MAX_MEASURE, MIN_CONTENT_WIDTH_RANGE, MIN_CONTENT_ZOOM,
    MIN_FONT_SIZE, MIN_LINE_HEIGHT, MIN_MEASURE, ZOOM_STEP,
};
use crate::events::SET_CONTENT_ZOOM_IN_WINDOW;
use dioxus::desktop::tao::window::WindowId;
use dioxus::prelude::*;

/// Line lengths offered by name. Standard is the measure most books settle
/// on; Wide is the length the page has always had.
const MEASURE_PRESETS: [(&str, f64); 3] = [("狭い", 40.0), ("標準", 50.0), ("広い", 60.0)];

const SAMPLE_LATIN: &str = "The quick brown fox jumps over the lazy dog. A line that runs too long loses the eye on its way back to the start of the next; one that is too short breaks the sentence into pieces.";

// Drawn in the faces of the CJK font language chosen, as a document's are,
// so what it does to Han characters shows here.
// The last line gathers characters whose glyphs differ most between the
// languages.
const SAMPLES_CJK: [&str; 4] = [
    "吾輩は猫である。名前はまだ無い。行が長すぎると次の行頭を見失い、短すぎると文が細切れになる。",
    "行太长，视线就难以回到下一行的开头；行太短，句子又会被切得支离破碎。",
    "줄이 너무 길면 다음 줄의 시작을 놓치고, 너무 짧으면 문장이 조각난다.",
    "直 骨 角 今 令 户 雪 画 海 次",
];

/// CJK font languages offered by name, in the order they are offered.
const LANGUAGES: [(CjkFontLanguage, &str); 5] = [
    (CjkFontLanguage::Auto, "自動"),
    (CjkFontLanguage::Ja, "日本語"),
    (CjkFontLanguage::ZhHans, "简体中文"),
    (CjkFontLanguage::ZhHant, "繁體中文"),
    (CjkFontLanguage::Ko, "한국어"),
];

/// A line length told as the characters it holds: an em is one full-width
/// character or about two half-width ones.
fn measure_hint(measure: f64) -> String {
    let full_width = normalize_measure(measure) as u32;
    format!(
        "1行あたり全角約{full_width}文字、半角約{}文字です。",
        full_width * 2
    )
}

/// The page itself: how large it is set, how much width it keeps, and what is
/// left in the margin beside it.
#[component]
pub fn ReadingTab(
    config: Signal<Config>,
    /// The window the "Current Settings" section acts on.
    window_id: WindowId,
    /// That window's zoom level, which this pane both shows and sets.
    mut current_zoom: Signal<f64>,
) -> Element {
    let zoom_cfg = config.read().zoom.clone();
    let sidebar_cfg = config.read().sidebar.clone();
    let typography_cfg = config.read().typography.clone();
    let reading_cfg = config.read().reading.clone();
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
                    p { class: "preference-description", "現在のウィンドウの本文拡大率です。" }
                }
                SliderInput {
                    value: current_zoom(),
                    min: MIN_CONTENT_ZOOM,
                    max: MAX_CONTENT_ZOOM,
                    step: ZOOM_STEP,
                    unit: "x".to_string(),
                    decimals: 1,
                    on_change: move |new_zoom| {
                        let normalized = normalize_content_zoom(new_zoom);
                        current_zoom.set(normalized);
                        let _ = SET_CONTENT_ZOOM_IN_WINDOW.send((window_id, normalized));
                    },
                    default_value: Some(zoom_cfg.default_zoom_level),
                }
            }

            h3 { class: "preference-section-title", "既定の設定" }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "既定の拡大率" }
                    p { class: "preference-description", "ウィンドウを開くときの本文拡大率です。" }
                }
                SliderInput {
                    value: zoom_cfg.default_zoom_level,
                    min: MIN_CONTENT_ZOOM,
                    max: MAX_CONTENT_ZOOM,
                    step: ZOOM_STEP,
                    unit: "x".to_string(),
                    decimals: 1,
                    on_change: move |new_zoom| {
                        config.write().zoom.default_zoom_level = new_zoom;
                    },
                    current_value: Some(current_zoom()),
                    shipped: Some(defaults.zoom.default_zoom_level),
                }
            }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "本文の最小幅" }
                    p {
                        class: "preference-description",
                        "本文が保つ最小の幅です。余白の履歴、サイドバー、目次、レールの順に、この幅を優先して折りたたまれます。"
                    }
                }
                SliderInput {
                    value: sidebar_cfg.min_content_width,
                    min: *MIN_CONTENT_WIDTH_RANGE.start(),
                    max: *MIN_CONTENT_WIDTH_RANGE.end(),
                    step: 10.0,
                    unit: "px".to_string(),
                    on_change: move |new_width| {
                        config.write().sidebar.min_content_width = new_width;
                    },
                    shipped: Some(defaults.sidebar.min_content_width),
                }
            }

            h3 { class: "preference-section-title", "文字組み" }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "行の長さ" }
                    p {
                        class: "preference-description",
                        "本文1行の最大幅をem単位で指定します。{measure_hint(typography_cfg.measure)}"
                    }
                }
                SliderInput {
                    value: typography_cfg.measure,
                    min: MIN_MEASURE,
                    max: MAX_MEASURE,
                    step: 1.0,
                    unit: "em".to_string(),
                    on_change: move |measure| {
                        config.write().typography.measure = normalize_measure(measure);
                    },
                    shipped: Some(defaults.typography.measure),
                }
            }

            ChoiceRow {
                name: "reading-measure-preset".to_string(),
                label: "行の長さのプリセット".to_string(),
                description: None,
                options: MEASURE_PRESETS
                    .iter()
                    .map(|(label, measure)| ChoiceItem {
                        value: *measure,
                        label: label.to_string(),
                    })
                    .collect(),
                selected: typography_cfg.measure,
                on_change: move |measure| {
                    config.write().typography.measure = measure;
                },
            }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "行間" }
                    p { class: "preference-description", "文字サイズに対する行の高さです。長い行や日本語では広めにすると読みやすくなります。" }
                }
                SliderInput {
                    value: typography_cfg.line_height,
                    min: MIN_LINE_HEIGHT,
                    max: MAX_LINE_HEIGHT,
                    step: LINE_HEIGHT_STEP,
                    unit: String::new(),
                    decimals: 2,
                    on_change: move |line_height| {
                        config.write().typography.line_height = normalize_line_height(line_height);
                    },
                    shipped: Some(defaults.typography.line_height),
                }
            }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "書体" }
                    p { class: "preference-description", "本文の書体です。コードは等幅フォントのままです。" }
                }
                OptionCards {
                    name: "reading-font-family".to_string(),
                    options: vec![
                        OptionCardItem {
                            icon: None,
                            value: FontFamilyChoice::Sans,
                            title: "サンセリフ".to_string(),
                            description: Some("GitHubと同じ書体".to_string()),
                        },
                        OptionCardItem {
                            icon: None,
                            value: FontFamilyChoice::Serif,
                            title: "セリフ".to_string(),
                            description: Some("書籍向けの書体".to_string()),
                        },
                        OptionCardItem {
                            icon: None,
                            value: FontFamilyChoice::Mono,
                            title: "等幅".to_string(),
                            description: Some("すべての文字幅を揃える".to_string()),
                        },
                        OptionCardItem {
                            icon: None,
                            value: FontFamilyChoice::Custom,
                            title: "カスタム".to_string(),
                            description: Some("独自のfont-familyを指定".to_string()),
                        },
                    ],
                    selected: typography_cfg.font_family,
                    on_change: move |font_family| {
                        config.write().typography.font_family = font_family;
                    },
                    shipped: Some(defaults.typography.font_family),
                }
                if typography_cfg.font_family == FontFamilyChoice::Custom {
                    div {
                        class: "typography-custom-font",
                        input {
                            r#type: "text",
                            spellcheck: false,
                            placeholder: "\"Source Han Serif\", serif",
                            value: "{typography_cfg.custom_font_family}",
                            oninput: move |event| {
                                config.write().typography.custom_font_family = event.value();
                            },
                        }
                        if typography_cfg.font_stack().is_none() {
                            p {
                                class: "preference-description",
                                "使用できないため、本文はサンセリフのままです。フォント名はカンマ区切りで指定してください。"
                            }
                        }
                    }
                }
            }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "文字サイズ" }
                    p { class: "preference-description", "本文の文字サイズです。拡大率は画像を含むページ全体に適用されます。" }
                }
                SliderInput {
                    value: typography_cfg.font_size,
                    min: MIN_FONT_SIZE,
                    max: MAX_FONT_SIZE,
                    step: 1.0,
                    unit: "px".to_string(),
                    on_change: move |font_size| {
                        config.write().typography.font_size = normalize_font_size(font_size);
                    },
                    shipped: Some(defaults.typography.font_size),
                }
            }

            ChoiceRow {
                name: "reading-cjk-font-language".to_string(),
                label: "CJKフォントの言語".to_string(),
                description: Some("中国語、日本語、韓国語の文字をどの言語向け書体で表示するかを選びます。自動ではシステム設定を使います。".to_string()),
                options: LANGUAGES
                    .iter()
                    .map(|(language, label)| ChoiceItem {
                        value: *language,
                        label: label.to_string(),
                    })
                    .collect(),
                selected: typography_cfg.cjk_font_language,
                on_change: move |language| {
                    config.write().typography.cjk_font_language = language;
                },
                shipped: Some(defaults.typography.cjk_font_language),
            }

            div {
                class: "typography-sample",
                style: "{typography_cfg.css_declarations()}",
                div {
                    class: "markdown-body",
                    p { lang: "en", "{SAMPLE_LATIN}" }
                    for text in SAMPLES_CJK {
                        p { "{text}" }
                    }
                }
            }

            h3 { class: "preference-section-title", "余白の履歴" }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "表示するタイミング" }
                    p {
                        class: "preference-description",
                        "このドキュメントより前に読んだ項目を、ページ左端の余白に表示します。"
                    }
                }
                OptionCards {
                    name: "reading-recent-trace".to_string(),
                    options: vec![
                        OptionCardItem {
                            icon: None,
                            value: RecentTrace::Never,
                            title: "表示しない".to_string(),
                            description: Some("余白を空けたままにします".to_string()),
                        },
                        OptionCardItem {
                            icon: None,
                            value: RecentTrace::HiddenWhenSidebar,
                            title: "サイドバー表示中以外".to_string(),
                            description: Some("サイドバーが出ている間は隠します".to_string()),
                        },
                        OptionCardItem {
                            icon: None,
                            value: RecentTrace::Always,
                            title: "常に表示".to_string(),
                            description: Some("表示できる幅があれば常に表示します".to_string()),
                        },
                    ],
                    selected: sidebar_cfg.recent_trace,
                    on_change: move |new_trace| {
                        config.write().sidebar.recent_trace = new_trace;
                    },
                    shipped: Some(defaults.sidebar.recent_trace),
                }
            }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "履歴に表示するドキュメント数" }
                    p { class: "preference-description", "余白の履歴に表示するドキュメント数です。" }
                }
                SliderInput {
                    value: sidebar_cfg.recent_trace_count as f64,
                    min: 1.0,
                    max: 12.0,
                    step: 1.0,
                    unit: String::new(),
                    on_change: move |new_count: f64| {
                        config.write().sidebar.recent_trace_count = new_count.max(1.0) as usize;
                    },
                    shipped: Some(defaults.sidebar.recent_trace_count as f64),
                }
            }

            h3 { class: "preference-section-title", "読了時間" }

            ToggleRow {
                label: "読了時間を表示".to_string(),
                description: Some("読了までの目安と残り時間をヘッダーに表示します。".to_string()),
                checked: reading_cfg.show_time,
                on_change: move |on| config.write().reading.show_time = on,
                shipped: Some(defaults.reading.show_time),
            }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "1分あたりの単語数" }
                    p { class: "preference-description", "英語など、単語で読む文章の読書速度です。" }
                }
                SliderInput {
                    value: reading_cfg.words_per_minute as f64,
                    min: 100.0,
                    max: 600.0,
                    step: 10.0,
                    unit: String::new(),
                    on_change: move |speed: f64| {
                        config.write().reading.words_per_minute = speed.max(1.0) as u32;
                    },
                    shipped: Some(defaults.reading.words_per_minute as f64),
                }
            }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "1分あたりの文字数" }
                    p { class: "preference-description", "中国語、日本語、韓国語の読書速度です。" }
                }
                SliderInput {
                    value: reading_cfg.characters_per_minute as f64,
                    min: 200.0,
                    max: 1200.0,
                    step: 10.0,
                    unit: String::new(),
                    on_change: move |speed: f64| {
                        config.write().reading.characters_per_minute = speed.max(1.0) as u32;
                    },
                    shipped: Some(defaults.reading.characters_per_minute as f64),
                }
            }

            div {
                class: "preference-item",
                div {
                    class: "preference-item-header",
                    label { "読了時間を表示する最短時間" }
                    p { class: "preference-description", "この時間未満で読み終えるドキュメントには読了時間を表示しません。" }
                }
                SliderInput {
                    value: reading_cfg.min_minutes as f64,
                    min: 0.0,
                    max: 30.0,
                    step: 1.0,
                    unit: " 分".to_string(),
                    on_change: move |minutes: f64| {
                        config.write().reading.min_minutes = minutes.max(0.0) as u32;
                    },
                    shipped: Some(defaults.reading.min_minutes as f64),
                }
            }

            h3 { class: "preference-section-title", "前回読んだ後の変更" }

            ToggleRow {
                label: "変更箇所を表示".to_string(),
                description: Some("前回読んだ後に追加・更新・削除された箇所を表示します。".to_string()),
                checked: reading_cfg.show_changes,
                on_change: move |on| config.write().reading.show_changes = on,
                shipped: Some(defaults.reading.show_changes),
            }

            ToggleRow {
                label: "空白の変更を無視".to_string(),
                description: Some("空白だけが異なる行を変更として扱いません。".to_string()),
                checked: reading_cfg.ignore_whitespace_changes,
                on_change: move |on| config.write().reading.ignore_whitespace_changes = on,
                shipped: Some(defaults.reading.ignore_whitespace_changes),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_measure_is_told_in_characters_of_either_width() {
        assert_eq!(
            measure_hint(60.0),
            "1行あたり全角約60文字、半角約120文字です。"
        );
    }

    #[test]
    fn the_measure_hint_names_the_length_the_page_will_use() {
        assert_eq!(
            measure_hint(500.0),
            "1行あたり全角約100文字、半角約200文字です。"
        );
    }
}
