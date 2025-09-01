use std::{
    sync::mpsc::{Receiver, Sender, TryRecvError},
    time::Duration,
};

use singlyton::SingletonUninit;
use tokio::sync::watch;
use tracing::debug;

use crate::state::{
    background_result::BackgroundResult, data_source_state::SignalGeneratorProperty,
    ui_command::UiCommand,
};

/// 后台线程的sender和receiver
pub static SEND_TO_UI: SingletonUninit<Sender<BackgroundResult>> = SingletonUninit::uninit();
pub static RECEIVE_UI_MESSAGE: SingletonUninit<Receiver<UiCommand>> = SingletonUninit::uninit();

/// 接收Ui发送的消息并作相应处理
pub fn background_task_dispatcher() {
    // 创建tokio runtime
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let _runtime_guard = runtime.enter();
    // false 表示未停止
    let (tx, rx) = watch::channel(false);
    loop {
        match RECEIVE_UI_MESSAGE.get().try_recv() {
            Ok(command) => {
                debug!("Ui Command:{command:#?}");
                //根据Ui的消息作相应处理
                match command {
                    UiCommand::StartRandomGenerator(signal_generator_property) => {
                        //开一个线程一直发数据
                        tokio::spawn(generate_random_data(signal_generator_property, rx.clone()));
                    }
                    UiCommand::StopDataGenerator => {
                        let _ = tx.send(true);
                    }
                    UiCommand::StopBackgroundManager => {
                        debug!("收到Ui的停止后台线程命令");
                        break;
                    }
                }
            }
            Err(e) => match e {
                TryRecvError::Empty => {
                    //这个错误不需要处理
                }
                TryRecvError::Disconnected => panic!("未创建和Ui线程的mpsc"),
            },
        }
    }
    runtime.shutdown_background();
}

async fn generate_random_data(
    signal_generator_property: SignalGeneratorProperty,
    stop_signal: watch::Receiver<bool>,
) {
    loop {
        // 检查是否收到停止信号
        if *stop_signal.borrow() {
            debug!("收到停止信号，退出循环");
            break;
        }

        let _ = SEND_TO_UI.get().send(BackgroundResult::TestResult);
        tokio::time::sleep(Duration::from_millis(signal_generator_property.interval)).await;
    }
}
