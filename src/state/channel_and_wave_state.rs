use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct ChannelAndWaveState {
    /// Ui和后台线程共享
    all_waves: Arc<Mutex<Vec<SingleWave>>>,
}

/// 每条要画的波形的属性
///
/// # Fields
///
/// - `channel_name` (`String`) - 通道名称
/// - `offset` (`f64`) - 距离x轴的偏移,为了把多通道波形分开
/// - `data` (`Vec<f64>`) - 数据(可能是采样过的)
///
pub struct SingleWave {
    pub channel_name: String,
    pub offset: f64,
    pub data: Vec<f64>,
}
