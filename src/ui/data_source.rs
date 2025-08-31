use egui::{Context, Ui};
use tracing::info;

use crate::{
    state::{
        app_state::AppState,
        data_source_state::{DataGeneratorType, DataSourceMode},
        ui_command::UiCommand,
    },
    ui::ui_mspc::send_to_background,
};

/// 选择数据源显示的内容
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
/// let _ = show_data_source();
/// ```
pub fn show_data_source(_ctx: &Context, ui: &mut Ui, app_state: &mut AppState) {
    ui.horizontal(|ui| {
        egui::ComboBox::from_label("选择数据来源")
            .selected_text(format!("{}", app_state.data_source_state.current_mode))
            .show_ui(ui, |ui| {
                if ui
                    .selectable_value(
                        &mut app_state.data_source_state.current_mode,
                        DataSourceMode::SignalGenerator,
                        "信号发生器",
                    )
                    .clicked()
                {
                    info!("切换到信号发生器模式");
                };
                if ui
                    .selectable_value(
                        &mut app_state.data_source_state.current_mode,
                        DataSourceMode::Socket,
                        "Socket",
                    )
                    .clicked()
                {
                    info!("切换到Socket模式");
                };
                if ui
                    .selectable_value(
                        &mut app_state.data_source_state.current_mode,
                        DataSourceMode::File,
                        "BDF文件",
                    )
                    .clicked()
                {
                    info!("切换到BDF文件模式");
                };
            });
        ui.separator();
        // 根据不同模式显示不同内容
        match app_state.data_source_state.current_mode {
            crate::state::data_source_state::DataSourceMode::SignalGenerator => {
                show_signal_generator(ui, app_state);
            }
            crate::state::data_source_state::DataSourceMode::Socket => {
                ui.label("Socket");
            }
            crate::state::data_source_state::DataSourceMode::File => {
                ui.label("File");
            }
        };
    });
}

/// 选择信号发生器时显示的内容
///
/// # Arguments
///
/// - `ui` (`&mut Ui`) - Describe this parameter.
/// - `app_state` (`&mut AppState`) - Describe this parameter.
///
/// # Examples
///
/// ```
/// use crate::...;
///
/// let _ = show_signal_generator();
/// ```
fn show_signal_generator(ui: &mut Ui, app_state: &mut AppState) {
    egui::ComboBox::from_label("选择数据类型")
        .selected_text(format!(
            "{}",
            app_state
                .data_source_state
                .signal_generator_property
                .data_type
        ))
        .show_ui(ui, |ui| {
            if ui
                .selectable_value(
                    &mut app_state
                        .data_source_state
                        .signal_generator_property
                        .data_type,
                    DataGeneratorType::Sine,
                    "正弦波",
                )
                .clicked()
            {
                info!("选择正弦波");
            };
            if ui
                .selectable_value(
                    &mut app_state
                        .data_source_state
                        .signal_generator_property
                        .data_type,
                    DataGeneratorType::Square,
                    "方波",
                )
                .clicked()
            {
                info!("选择方波");
            };
            if ui
                .selectable_value(
                    &mut app_state
                        .data_source_state
                        .signal_generator_property
                        .data_type,
                    DataGeneratorType::Sawtooth,
                    "锯齿波",
                )
                .clicked()
            {
                info!("选择锯齿波");
            };
            if ui
                .selectable_value(
                    &mut app_state
                        .data_source_state
                        .signal_generator_property
                        .data_type,
                    DataGeneratorType::Random,
                    "随机数",
                )
                .clicked()
            {
                info!("选择随机数");
            };
        });
    ui.separator();
    match app_state
        .data_source_state
        .signal_generator_property
        .data_type
    {
        DataGeneratorType::Sine => {
            ui.label("正弦波");
        }
        DataGeneratorType::Square => {
            ui.label("方波");
        }
        DataGeneratorType::Sawtooth => {
            ui.label("锯齿波");
        }
        DataGeneratorType::Random => show_random_generator(ui, app_state),
    };
    if ui.button("Start").clicked() {
        send_to_background(UiCommand::StartRandomGenerator(
            app_state
                .data_source_state
                .signal_generator_property
                .clone(),
        ));
        info!("开始产生数据");
    }
    if ui.button("Stop").clicked() {
        send_to_background(UiCommand::StopDataGenerator);
        info!("停止产生数据");
    }
}

/// 选择随机数时显示的内容
///
/// # Arguments
///
/// - `ui` (`&mut Ui`) - Describe this parameter.
/// - `app_state` (`&mut AppState`) - Describe this parameter.
///
/// # Examples
///
/// ```
/// use crate::...;
///
/// let _ = show_random_generator();
/// ```
fn show_random_generator(ui: &mut Ui, app_state: &mut AppState) {
    ui.add(
        egui::Slider::new(
            &mut app_state
                .data_source_state
                .signal_generator_property
                .channel_number,
            1..=64,
        )
        .text("通道数"),
    );
    ui.add(
        egui::Slider::new(
            &mut app_state
                .data_source_state
                .signal_generator_property
                .data_length,
            500..=4000,
        )
        .text("数据长度"),
    );
    ui.add(
        egui::Slider::new(
            &mut app_state
                .data_source_state
                .signal_generator_property
                .interval,
            5..=100,
        )
        .text("数据产生的间隔(ms)"),
    );
}
