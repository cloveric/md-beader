use markdown_reader::ui::{ReaderTheme, metrics_for_theme};

#[test]
fn has_three_reading_theme_presets() {
    let all = ReaderTheme::all();
    assert_eq!(all.len(), 3);
    assert_eq!(all[0].label(), "GitHub 风");
    assert_eq!(all[1].label(), "Typora 风");
    assert_eq!(all[2].label(), "紧凑风");
}

#[test]
fn compact_theme_is_denser_than_others() {
    let github = metrics_for_theme(ReaderTheme::GitHub);
    let typora = metrics_for_theme(ReaderTheme::Typora);
    let compact = metrics_for_theme(ReaderTheme::Compact);

    assert!(compact.body_line_height < github.body_line_height);
    assert!(compact.body_line_height < typora.body_line_height);
    assert!(compact.paragraph_gap < github.paragraph_gap);
}
