use rand::{Rng, SeedableRng, rngs::StdRng};
use singlyton::SingletonUninit;
use std::{
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender, TryRecvError},
    },
    time::Duration,
};
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
                    UiCommand::StartRandomGenerator(signal_generator_property, all_waves) => {
                        // 开一个线程一直发数据
                        // 先把停止状态取消
                        let _ = tx.send(false);
                        tokio::spawn(generate_random_data(
                            signal_generator_property,
                            all_waves,
                            rx.clone(),
                        ));
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
    all_waves: Arc<Mutex<Vec<SingleWave>>>,
    stop_signal: watch::Receiver<bool>,
) {
    // 先创建好空结果
    let channel_number = signal_generator_property.channel_number as usize;
    let data_length = signal_generator_property.data_length as usize;
    {
        let mut locked = all_waves.lock().unwrap();
        locked.clear();
        for i in 0..channel_number {
            locked.push(SingleWave {
                channel_name: format!("CH{}", i + 1),
                offset: i as f64 + 0.5,
                data: (0..data_length).map(|i| [i as f64 * 0.001, 0.0]).collect(),
            });
        }
    }
    let mut rng = {
        let mut rng = rand::rng();
        StdRng::from_rng(&mut rng)
    };
    let mut idx = 0;
    loop {
        if *stop_signal.borrow() {
            debug!("收到停止信号，退出循环");
            break;
        }
        {
            let mut locked = all_waves.lock().unwrap();
            for ch in 0..channel_number {
                locked[ch].data[idx] = [idx as f64 * 0.001, rng.random::<f64>() + ch as f64];
            }
        }
        idx = (idx + 1) % data_length;
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
}
