use std::fs;
use std::path::PathBuf;

use markdown_reader::ui::{
    default_cjk_font_candidates, default_monospace_font_candidates, default_ui_font_candidates,
    first_existing_path,
};

#[test]
fn picks_first_existing_font_path() {
    let base = std::env::temp_dir().join("md_reader_font_pick_test");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).expect("create temp dir");

    let p1 = base.join("missing.ttf");
    let p2 = base.join("exists_a.ttf");
    let p3 = base.join("exists_b.ttf");
    fs::write(&p2, b"dummy").expect("write p2");
    fs::write(&p3, b"dummy").expect("write p3");

    let picked = first_existing_path(&[p1.clone(), p2.clone(), p3.clone()]);
    assert_eq!(picked, Some(p2));

    let _ = fs::remove_dir_all(&base);
}

#[test]
fn has_common_windows_cjk_candidates() {
    let candidates = default_cjk_font_candidates();
    let expected = PathBuf::from(r"C:\Windows\Fonts\msyh.ttc");
    assert!(candidates.contains(&expected));
    assert_eq!(candidates.first(), Some(&expected));
}

#[cfg(windows)]
#[test]
fn has_available_cjk_font_on_this_machine() {
    let candidates = default_cjk_font_candidates();
    let found = first_existing_path(&candidates);
    assert!(
        found.is_some(),
        "no CJK font found in candidates: {candidates:?}"
    );
    assert_eq!(found, Some(PathBuf::from(r"C:\Windows\Fonts\msyh.ttc")));
}

#[test]
fn ui_font_prefers_microsoft_yahei_first() {
    let candidates = default_ui_font_candidates();
    assert_eq!(
        candidates.first(),
        Some(&PathBuf::from(r"C:\Windows\Fonts\msyh.ttc"))
    );
}

#[test]
fn monospace_font_prefers_consolas_first() {
    let candidates = default_monospace_font_candidates();
    assert_eq!(
        candidates.first(),
        Some(&PathBuf::from(r"C:\Windows\Fonts\consola.ttf"))
    );
}
