use std::sync::{Arc, Mutex};

use crate::state::{
    channel_and_wave_state::SingleWave, data_source_state::SignalGeneratorProperty,
};

/// Ui向后台发送的所有命令
///
/// # Variants
///
/// - `StartRandomGenerator(SignalGeneratorProperty)` - 根据SignalGeneratorProperty中的属性开始产生随机数
/// - `StopDataGenerator` - 停止产生数据
///
#[derive(Debug)]
pub enum UiCommand {
    StopBackgroundManager,
    StartRandomGenerator(SignalGeneratorProperty, Arc<Mutex<Vec<SingleWave>>>),
    StopDataGenerator,
}
