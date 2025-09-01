use crate::state::{
    channel_and_wave_state::ChannelAndWaveState, data_source_state::DataSourceState,
};

/// 整个Ui的状态
#[derive(Default)]
pub struct AppState {
    pub data_source_state: DataSourceState,
    pub channel_and_waves: ChannelAndWaveState,
}
