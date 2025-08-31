use egui::CentralPanel;
use tokio::sync::mpsc::error::TryRecvError;
use tracing::{debug, info};

use crate::state::app_state::AppState;
use crate::state::ui_command::UiCommand;
use crate::ui::ui_mspc::{RECEIVE_BACKGROUND_MESSAGE, send_to_background};

impl eframe::App for AppState {
    /// 每帧刷新时自动调用
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //每次重绘Ui前接收一下后台线程传过来的消息
        match RECEIVE_BACKGROUND_MESSAGE.get_mut().try_recv() {
            Ok(result) => {
                debug!("background result:{result:?}");
                //根据这个结果对Ui的state作响应修改
            }
            Err(e) => match e {
                TryRecvError::Empty => {
                    //这个错误不需要处理
                }
                TryRecvError::Disconnected => panic!("未创建和后台线程的mpsc"),
            },
        }
        CentralPanel::default().show(_ctx, |ui| {
            if ui.button("点击向后台发送命令").clicked() {
                send_to_background(UiCommand::TestCommand);
            }
        });
    }

    /// 主窗口关闭时触发
    fn on_exit(&mut self) {
        info!("程序关闭");
    }
}
