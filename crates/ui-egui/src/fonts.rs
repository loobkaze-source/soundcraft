//! Fonts: egui's bundled defaults, plus the operating system's own UI fonts loaded at runtime when
//! present (never redistributed), plus the bundled Noto Sans Thai (OFL-1.1, `assets/fonts/`) as a
//! fallback for Thai text. A "bold" family is always defined so the UI can use it.

use egui::{FontData, FontDefinitions, FontFamily};
use std::sync::Arc;

/// (path, ttc index) candidates for regular and bold UI text, by platform.
const REGULAR: &[(&str, u32)] = &[
    ("/System/Library/Fonts/HelveticaNeue.ttc", 0),
    ("/System/Library/Fonts/Helvetica.ttc", 0),
    ("C:\\Windows\\Fonts\\segoeui.ttf", 0),
    ("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 0),
    ("/usr/share/fonts/TTF/DejaVuSans.ttf", 0),
    ("/usr/share/fonts/dejavu/DejaVuSans.ttf", 0),
    ("/usr/local/share/fonts/dejavu/DejaVuSans.ttf", 0),
    ("/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf", 0),
];
const BOLD: &[(&str, u32)] = &[
    ("/System/Library/Fonts/HelveticaNeue.ttc", 1),
    ("/System/Library/Fonts/Helvetica.ttc", 1),
    ("C:\\Windows\\Fonts\\segoeuib.ttf", 0),
    ("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", 0),
    ("/usr/share/fonts/TTF/DejaVuSans-Bold.ttf", 0),
    ("/usr/share/fonts/dejavu/DejaVuSans-Bold.ttf", 0),
    ("/usr/local/share/fonts/dejavu/DejaVuSans-Bold.ttf", 0),
    ("/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf", 0),
];

/// Asks fontconfig for the desktop's UI font. Only TrueType/OpenType results are used: egui cannot
/// read Type 1 or bitmap fonts, which `fc-match` may return on minimal systems.
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn load_from_fontconfig(bold: bool) -> Option<FontData> {
    let pattern = if bold { "sans-serif:style=Bold" } else { "sans-serif" };
    let output = std::process::Command::new("fc-match").args(["--format=%{file}\\n%{index}\\n", pattern]).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8(output.stdout).ok()?;
    let mut lines = stdout.lines();
    let path = lines.next()?.trim();
    // The upper 16 bits carry a variable font's named instance; the face index is the lower 16.
    let index = lines.next()?.trim().parse::<u32>().ok()? & 0xFFFF;
    let bytes = std::fs::read(path).ok()?;
    let sfnt = matches!(bytes.get(..4), Some(b"\0\x01\0\0" | b"OTTO" | b"true" | b"ttcf"));
    if !sfnt {
        return None;
    }
    let mut font = FontData::from_owned(bytes);
    font.index = index;
    Some(font)
}

#[cfg(not(target_arch = "wasm32"))]
fn load(cands: &[(&str, u32)], bold: bool) -> Option<FontData> {
    if std::env::var_os("SOUNDCRAFT_NO_SYSTEM_FONTS").is_some() {
        return None;
    }
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    if let Some(font) = load_from_fontconfig(bold) {
        return Some(font);
    }
    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    let _ = bold;
    for (p, idx) in cands {
        if let Ok(bytes) = std::fs::read(p) {
            let mut fd = FontData::from_owned(bytes);
            fd.index = *idx;
            return Some(fd);
        }
    }
    None
}

#[cfg(target_arch = "wasm32")]
fn load(_: &[(&str, u32)], _: bool) -> Option<FontData> {
    None
}

/// Bundled Thai fallback faces (never the primary UI font).
const THAI_REGULAR: &str = "NotoSansThai-Regular";
const THAI_BOLD: &str = "NotoSansThai-Bold";
static THAI_REGULAR_TTF: &[u8] = include_bytes!("../../../assets/fonts/NotoSansThai-Regular.ttf");
static THAI_BOLD_TTF: &[u8] = include_bytes!("../../../assets/fonts/NotoSansThai-Bold.ttf");

/// Adds `name` as a fallback: after the primary fonts, before egui's emoji fonts (or last).
fn insert_fallback(family: &mut Vec<String>, name: &str) {
    let at = family.iter().position(|f| f == "NotoEmoji-Regular" || f == "emoji-icon-font").unwrap_or(family.len());
    family.insert(at, name.into());
}

pub fn definitions() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(THAI_REGULAR.into(), Arc::new(FontData::from_static(THAI_REGULAR_TTF)));
    fonts.font_data.insert(THAI_BOLD.into(), Arc::new(FontData::from_static(THAI_BOLD_TTF)));
    if let Some(f) = fonts.families.get_mut(&FontFamily::Proportional) {
        insert_fallback(f, THAI_REGULAR);
    }
    if let Some(f) = fonts.families.get_mut(&FontFamily::Monospace) {
        f.push(THAI_REGULAR.into());
    }
    let mut bold_family: Vec<String> = fonts.families.get(&FontFamily::Proportional).cloned().unwrap_or_default();
    if let Some(i) = bold_family.iter().position(|f| f == THAI_REGULAR) {
        bold_family[i] = THAI_BOLD.into();
    }
    if let Some(reg) = load(REGULAR, false) {
        fonts.font_data.insert("system-regular".into(), Arc::new(reg));
        if let Some(f) = fonts.families.get_mut(&FontFamily::Proportional) {
            f.insert(0, "system-regular".into());
        }
    }
    if let Some(b) = load(BOLD, true) {
        fonts.font_data.insert("system-bold".into(), Arc::new(b));
        bold_family.insert(0, "system-bold".into());
    } else if fonts.font_data.contains_key("system-regular") {
        bold_family.insert(0, "system-regular".into());
    }
    fonts.families.insert(FontFamily::Name("bold".into()), bold_family);
    fonts
}

pub fn install(ctx: &egui::Context) {
    ctx.set_fonts(definitions());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noto_sans_thai_is_a_fallback_never_primary() {
        let fonts = definitions();
        let pos = |fam: &FontFamily, name: &str| fonts.families[fam].iter().position(|f| f == name);
        let prop = FontFamily::Proportional;
        let thai = pos(&prop, THAI_REGULAR).expect("Thai in Proportional");
        assert!(thai > pos(&prop, "Ubuntu-Light").expect("egui default font"));
        assert!(pos(&prop, "NotoEmoji-Regular").is_none_or(|e| thai < e));
        let mono = FontFamily::Monospace;
        assert_eq!(pos(&mono, THAI_REGULAR), Some(fonts.families[&mono].len() - 1));
        let bold = FontFamily::Name("bold".into());
        assert!(pos(&bold, THAI_BOLD).is_some_and(|i| i > 0));
        assert!(pos(&bold, THAI_REGULAR).is_none());
        assert!(fonts.font_data.contains_key(THAI_REGULAR) && fonts.font_data.contains_key(THAI_BOLD));
    }
}
