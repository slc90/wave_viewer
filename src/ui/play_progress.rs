use egui::{Context, Ui};

/// 播放进度条显示的内容
///
/// # Arguments
///
/// - `_ctx` (`&Context`) - Describe this parameter.
/// - `ui` (`&mut Ui`) - Describe this parameter.
///
/// # Examples
///
/// ```
/// use crate::...;
///
/// let _ = show_play_progress();
/// ```
pub fn show_play_progress(_ctx: &Context, ui: &mut Ui) {
    ui.vertical_centered_justified(|ui| ui.label("Bottom"));
}
