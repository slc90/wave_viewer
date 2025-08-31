use egui::{Color32, Context, RichText, Ui};
use egui_plot::{Line, Plot, PlotPoint, PlotPoints, Text, VLine};

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
pub fn show_channel_and_wave(ctx: &Context, ui: &mut Ui) {
    // 告诉 egui 下帧必须重绘
    ctx.request_repaint();
    Plot::new("channel_and_wave")
        .allow_drag(false)
        .allow_axis_zoom_drag(false)
        .allow_zoom(false)
        .allow_double_click_reset(false)
        .allow_boxed_zoom(false)
        .allow_scroll(false)
        // 这里的最大值稍微比x轴的范围大一些,不然最后一个值不显示
        // 最小值比0小一些,左边要用来画通道名
        .default_x_bounds(-0.02, 1.000001)
        .show_grid(false)
        .x_axis_label("Time(s)")
        // y轴不要显示刻度
        .y_axis_formatter(|_, _| "".to_string())
        .cursor_color(Color32::TRANSPARENT)
        .show(ui, |plot_ui| {
            (0..=32).for_each(|i| {
                // 画通道名
                let channel_name = &("ch".to_string() + &i.to_string());
                plot_ui.text(Text::new(
                    "ch".to_string() + &i.to_string(),
                    PlotPoint::new(-0.01, i as f64),
                    RichText::new(channel_name).size(16.0),
                ));
                // 画波形
                let points: PlotPoints = (0..1000).map(|j| [j as f64 * 0.001, i as f64]).collect();
                let line = Line::new(channel_name, points).allow_hover(false);
                plot_ui.line(line);
                // 手动画x轴
                let x_axis_points: PlotPoints =
                    (0..1000).map(|j| [j as f64 * 0.001, 0.0]).collect();
                let x_axis = Line::new("x-axis", x_axis_points)
                    .color(Color32::BLACK)
                    .allow_hover(false);
                plot_ui.line(x_axis);
                // 画y轴
                plot_ui.vline(VLine::new("y-axis", 0.0).color(Color32::BLACK));
            });
        });
}
