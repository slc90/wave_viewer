// 关闭release版的控制台
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod background;
pub mod logger;
pub mod state;
pub mod ui;
pub mod utils;

use std::thread;

use tracing::info;

use crate::background::manager::{RECEIVE_UI_MESSAGE, SEND_TO_UI};
use crate::logger::init_log;
use crate::state::app_state::AppState;
use crate::state::ui_command::UiCommand;
use crate::ui::fonts::add_font;
use crate::ui::ui_mpsc::{RECEIVE_BACKGROUND_MESSAGE, SEND_TO_BACKGROUND, send_to_background};
use crate::utils::constants::{WINDOW_MIN_HEIGHT, WINDOW_MIN_WIDTH};

fn main() -> eframe::Result {
    let _file_logger_work_guards = init_log();
    info!("程序启动");
    // 创建两对mpsc管道,Ui和后台双向通信
    let (_send_to_ui, _receive_background_message) = std::sync::mpsc::channel();
    let (_send_to_background, _receive_ui_message) = std::sync::mpsc::channel();
    SEND_TO_BACKGROUND.init(_send_to_background);
    RECEIVE_BACKGROUND_MESSAGE.init(_receive_background_message);
    SEND_TO_UI.init(_send_to_ui);
    RECEIVE_UI_MESSAGE.init(_receive_ui_message);
    // 启动后台任务
    let background_manager_handle = thread::spawn(|| {
        background::manager::background_task_dispatcher();
    });
    // 启动UI
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            // 初始为1920*1080
            .with_inner_size([WINDOW_MIN_WIDTH, WINDOW_MIN_HEIGHT])
            // 最小也为1920*1080
            .with_min_inner_size([WINDOW_MIN_WIDTH, WINDOW_MIN_HEIGHT]),
        centered: true,
        ..Default::default()
    };
    let eframe_result = eframe::run_native(
        "wave_viewer",
        native_options,
        Box::new(|cc| {
            add_font(&cc.egui_ctx);
            Ok(Box::new(AppState::default()))
        }),
    );
    // 把后台线程停掉
    send_to_background(UiCommand::StopBackgroundManager);
    background_manager_handle.join().unwrap();
    //返回给操作系统的结果
    eframe_result
}
