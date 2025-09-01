use std::{
    sync::mpsc::{Receiver, Sender, TryRecvError},
    time::Duration,
};

use rand::Rng;
use singlyton::SingletonUninit;
use tokio::sync::watch;
use tracing::debug;

use crate::state::{
    background_result::BackgroundResult, channel_and_wave_state::SingleWave,
    data_source_state::SignalGeneratorProperty, ui_command::UiCommand,
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
                //根据Ui的消息作相应处理
                match command {
                    UiCommand::StartRandomGenerator(signal_generator_property) => {
                        // 开一个线程一直发数据
                        // 先把停止状态取消
                        let _ = tx.send(false);
                        tokio::spawn(generate_random_data(signal_generator_property, rx.clone()));
                    }
                    UiCommand::StopDataGenerator => {
                        // 发送停止
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
        if *stop_signal.borrow() {
            debug!("收到停止信号，退出循环");
            break;
        }
        let mut all_waves = Vec::<SingleWave>::new();
        (0..signal_generator_property.channel_number).for_each(|i| {
            let single_wave = SingleWave {
                channel_name: format!("Ch{}", i + 1),
                offset: i as f64 + 0.5,
                data: (0..signal_generator_property.data_length)
                    .map(|j| {
                        let mut rng = rand::rng();
                        let value = rng.random::<f64>() + i as f64;
                        [j as f64 * 0.001, value]
                    })
                    .collect(),
            };
            all_waves.push(single_wave);
        });
        let _ = SEND_TO_UI
            .get()
            .send(BackgroundResult::ChannelAndWaves(all_waves));
        // 每20ms发一次结果,固定16ms画一帧,发太快也没意义
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}
