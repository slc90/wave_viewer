pub mod background;
pub mod logger;
pub mod state;
pub mod ui;
pub mod utils;

use tokio::sync::mpsc;
use tokio::task;
use tracing::info;

use crate::background::manager::{RECEIVE_UI_MESSAGE, SEND_TO_UI};
use crate::logger::init_log;
use crate::state::app_state::AppState;
use crate::ui::fonts::add_font;
use crate::ui::ui_mspc::{RECEIVE_BACKGROUND_MESSAGE, SEND_TO_BACKGROUND};
use crate::utils::constants::{WINDOW_MIN_HEIGHT, WINDOW_MIN_WIDTH};

fn main() -> eframe::Result {
    let _file_logger_work_guards = init_log();
    info!("程序启动");
    // 创建tokio runtime
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let _runtime_guard = runtime.enter();
    // 创建两对mpsc管道,Ui和后台双向通信
    let (send_to_ui, receive_background_message) = mpsc::channel(100);
    let (send_to_background, receive_ui_message) = mpsc::channel(100);
    SEND_TO_BACKGROUND.init(send_to_background);
    RECEIVE_BACKGROUND_MESSAGE.init(receive_background_message);
    SEND_TO_UI.init(send_to_ui);
    RECEIVE_UI_MESSAGE.init(receive_ui_message);
    // 启动后台任务
    task::spawn(async move {
        background::manager::background_task_dispatcher().await;
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
            Ok(Box::new(AppState))
        }),
    );
    //把后台线程都关掉
    runtime.shutdown_background();
    //返回给操作系统的结果
    eframe_result
}
