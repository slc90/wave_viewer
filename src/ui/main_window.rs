use egui::{CentralPanel, TopBottomPanel};
use tokio::sync::mpsc::error::TryRecvError;
use tracing::{debug, info};

use crate::state::app_state::AppState;
use crate::ui::channel_and_wave::show_channel_and_wave;
use crate::ui::data_source::show_data_source;
use crate::ui::play_progress::show_play_progress;
use crate::ui::ui_mspc::RECEIVE_BACKGROUND_MESSAGE;

impl eframe::App for AppState {
    /// 每帧刷新时自动调用
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
        // 顶部选择数据源
        TopBottomPanel::top("data_source_mode")
            .resizable(false)
            .show(ctx, |ui| {
                show_data_source(ctx, ui, self);
            });
        // 底部是播放相关选项
        TopBottomPanel::bottom("play_progress")
            .resizable(false)
            .show(ctx, |ui| {
                show_play_progress(ctx, ui);
            });
        // 中间是画图区域
        CentralPanel::default().show(ctx, |ui| {
            show_channel_and_wave(ctx, ui);
        });
    }

    /// 主窗口关闭时触发
    fn on_exit(&mut self) {
        info!("程序关闭");
    }
}
