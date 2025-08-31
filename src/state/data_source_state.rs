use strum_macros::Display;

#[derive(Default)]
pub struct DataSourceState {
    pub current_mode: DataSourceMode,
    pub signal_generator_property: SignalGeneratorProperty,
}

/// 数据源的几种模式
///
/// # Variants
///
/// - `SignalGenerator` - 程序自己产生数据
/// - `Socket` - 从Socket接收数据
/// - `File` - 从文件读取数据
#[derive(Debug, PartialEq, Display)]
pub enum DataSourceMode {
    #[strum(serialize = "信号发生器")]
    SignalGenerator,

    #[strum(serialize = "Socket")]
    Socket,

    #[strum(serialize = "BDF文件")]
    File,
}

impl Default for DataSourceMode {
    fn default() -> Self {
        DataSourceMode::SignalGenerator
    }
}

/// 信号发生器需要设置的一些属性
///
/// # Fields
///
/// - `data_type` (`DataGeneratorType`) - 产生的数据类型
/// - `channel_number` (`i64`) - 通道数量
/// - `data_length` (`i64`) - 数据点数
/// - `interval` (`i64`) - 产生数据的间隔,单位ms
///
/// # Examples
///
/// ```
/// use crate::...;
///
/// let s = SignalGeneratorProperty {
///     data_type: value,
///     channel_number: value,
///     data_length: value,
///     interval: value,
/// };
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct SignalGeneratorProperty {
    pub data_type: DataGeneratorType,
    pub channel_number: i64,
    pub data_length: i64,
    pub interval: i64,
}

impl Default for SignalGeneratorProperty {
    fn default() -> Self {
        SignalGeneratorProperty {
            data_type: DataGeneratorType::Random,
            channel_number: 32,
            data_length: 1000,
            interval: 5,
        }
    }
}

/// 信号发生器的类型
///
/// # Variants
///
/// - `Sine` - 正弦波
/// - `Square` - 方波
/// - `Sawtooth` - 锯齿波
/// - `Random` - 随机数
#[derive(Debug, PartialEq, Display, Clone)]
pub enum DataGeneratorType {
    #[strum(serialize = "正弦波")]
    Sine,

    #[strum(serialize = "方波")]
    Square,

    #[strum(serialize = "锯齿波")]
    Sawtooth,

    #[strum(serialize = "随机数")]
    Random,
}

impl Default for DataGeneratorType {
    fn default() -> Self {
        DataGeneratorType::Random
    }
}
