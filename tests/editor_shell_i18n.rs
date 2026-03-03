use std::fs;
use std::path::PathBuf;

fn shell_html() -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path = root.join("assets").join("editor_shell.html");
    fs::read_to_string(path).expect("read editor_shell.html")
}

#[test]
fn has_language_selector_control() {
    let html = shell_html();
    assert!(
        html.contains("id=\"langSelect\""),
        "expected a language selector element with id=langSelect"
    );
}

#[test]
fn includes_english_and_chinese_ui_dictionary() {
    let html = shell_html();
    assert!(
        html.contains("const UI_TEXT ="),
        "expected UI_TEXT dictionary in editor shell"
    );
    assert!(html.contains("Open"), "expected English UI labels");
    assert!(html.contains("打开"), "expected Chinese UI labels");
}

#[test]
fn defaults_to_english_without_saved_locale() {
    let html = shell_html();
    assert!(
        html.contains("const storedLocale = readStoredLocale();"),
        "expected bootstrap variable for saved locale"
    );
    assert!(
        html.contains("let appLocale = storedLocale ? normalizeLocale(storedLocale) : \"en\";"),
        "expected default locale to be English when no saved preference exists"
    );
}
