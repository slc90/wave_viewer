use crate::state::channel_and_wave_state::SingleWave;

/// 后台返回给Ui的所有结果
///
/// # Variants
///
/// - `TestResult` - 测试用结果
#[derive(Debug)]
pub enum BackgroundResult {
    ChannelAndWaves(Vec<SingleWave>),
}
