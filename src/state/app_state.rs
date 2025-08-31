use crate::state::data_source_state::DataSourceState;

/// 整个Ui的状态
#[derive(Default)]
pub struct AppState {
    pub data_source_state: DataSourceState,
}
