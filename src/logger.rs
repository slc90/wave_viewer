use std::{
    fs,
    path::{Path, PathBuf},
};

use chrono::Local;
use tracing::{Level, Metadata, Subscriber};
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{
    Layer,
    filter::FilterFn,
    fmt::{self, time::FormatTime},
    layer::SubscriberExt,
    registry::LookupSpan,
    util::SubscriberInitExt,
};

use crate::utils::{constants::LOG_FOLDER, utils::get_current_time_format_string};

/// 初始化所有console和file的log,要把word_guard返回并一直用一个变量hold住
pub fn init_log() -> [WorkerGuard; 3] {
    let current_log_folder = create_log_folder();
    let (debug_log, _debug_guard) = init_file_log(&current_log_folder, Level::DEBUG);
    let (info_log, _info_guard) = init_file_log(&current_log_folder, Level::INFO);
    let (error_log, _error_guard) = init_file_log(&current_log_folder, Level::ERROR);
    let layers = tracing_subscriber::registry()
        .with(debug_log)
        .with(info_log)
        .with(error_log);
    // 开发期间也把log输出到console
    #[cfg(debug_assertions)]
    {
        let console_log = init_console_log();
        layers.with(console_log).init();
    }
    //最终发布版不输出到console
    #[cfg(not(debug_assertions))]
    {
        layers.init();
    }
    [_debug_guard, _info_guard, _error_guard]
}

/// 这一次启动程序时使用的log文件夹
///
/// # Returns
///
/// - `PathBuf` - 这次创建好的文件夹路径
///
/// # Examples
///
/// ```
/// use path::{Path, PathBuf};
///
/// let current_log_folder = create_log_folder();
/// ```
fn create_log_folder() -> PathBuf {
    let current_time = get_current_time_format_string();
    let current_log_folder = Path::new(LOG_FOLDER).join(current_time);
    match fs::create_dir_all(&current_log_folder) {
        Ok(_) => current_log_folder,
        Err(err) => panic!("创建log文件夹失败,路径{current_log_folder:?},原因:{err}"),
    }
}

/// 用于日志中时间的格式化
struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = Local::now();
        write!(w, "{}", now.format("%Y-%m-%d-%H:%M:%S%.3f"))
    }
}

/// 初始化控制台log,输出Debug级别以上的log(不输出Trace级别,太多了)
///
/// # Arguments
///
/// # Returns
///
/// - `impl Layer<Registry>` - 用tracing_subscriber::registry().with()注册这个layer.
///
/// # Examples
///
/// ```
/// use tracing_subscriber;
///
/// let console_layer = init_console_log();
/// tracing_subscriber::registry().with(console_log)
/// ```
#[cfg(debug_assertions)]
fn init_console_log<T>() -> impl Layer<T>
where
    T: Subscriber + for<'a> LookupSpan<'a>,
{
    let console_layer = fmt::layer()
        .with_timer(LocalTimer)
        .with_target(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_filter(FilterFn::new(move |metadata| {
            log_filter_function(&Level::DEBUG, &metadata)
        }));
    console_layer
}

/// 初始化各个级别的文件log
///
/// # Arguments
///
/// - `path` (`&PathBuf`) - 记录到的路径,不要带后缀名
/// - `level` (`Level`) - 日志的等级
///
/// # Returns
///
/// - `(impl Layer<T>, WorkerGuard) where T: Subscriber + for<'a> LookupSpan<'a>,`
/// - 用tracing_subscriber::registry().with()注册这个layer.
///
/// # Examples
///
/// ```
/// use tracing_subscriber;
///
/// let (layer, _guard) = init_file_log();
/// tracing_subscriber::registry().with(layer)
/// ```
fn init_file_log<T>(path: &PathBuf, level: Level) -> (impl Layer<T>, WorkerGuard)
where
    T: Subscriber + for<'a> LookupSpan<'a>,
{
    let file_appender = rolling::never(path, level.to_string() + ".log");
    let (writer, _guard) = tracing_appender::non_blocking(file_appender);
    let layer = fmt::layer()
        .with_target(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_timer(LocalTimer)
        .with_writer(writer)
        .with_ansi(false)
        .with_filter(FilterFn::new(move |metadata| {
            log_filter_function(&level, &metadata)
        }));
    (layer, _guard)
}

/// 文件日志过滤用的函数
/// 满足条件的日志才会被记录
/// # Arguments
///
/// - `level` (`&Level`) - 日志的等级
/// - `metadata` (`&Metadata<'_>`) - 闭包传进来的参数
///
/// # Returns
///
/// - `bool` -
fn log_filter_function(level: &Level, metadata: &Metadata<'_>) -> bool {
    let meta_level: &Level = metadata.level();
    //获取主程序自己的crate名称,用于过滤掉三方库
    //注意这里要用 <= ,详细见 https://docs.rs/tracing/latest/tracing/struct.Level.html#method.le
    let crate_name: &str = env!("CARGO_PKG_NAME");
    metadata.target().starts_with(crate_name) && meta_level <= level
}
