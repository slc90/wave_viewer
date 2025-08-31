use egui::{Context, Ui};

/// 显示通道和波形
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
/// let _ = show_channel_and_wave();
/// ```
pub fn show_channel_and_wave(_ctx: &Context, ui: &mut Ui) {
    ui.vertical_centered_justified(|ui| ui.label("Central"));
}
