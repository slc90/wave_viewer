use egui::epaint::text::{FontInsert, InsertFontFamily};

use crate::utils::constants::FONT_NAME;

/// 添加字体
/// 字体文件见 https://github.com/atelier-anchor/smiley-sans
pub fn add_font(ctx: &egui::Context) {
    ctx.add_font(FontInsert::new(
        FONT_NAME,
        egui::FontData::from_static(include_bytes!("../../fonts/SmileySans-Oblique.ttf")),
        vec![
            InsertFontFamily {
                family: egui::FontFamily::Proportional,
                priority: egui::epaint::text::FontPriority::Highest,
            },
            InsertFontFamily {
                family: egui::FontFamily::Monospace,
                priority: egui::epaint::text::FontPriority::Lowest,
            },
        ],
    ));
}
