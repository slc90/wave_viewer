use chrono::Local;

/// 获取当前时间，转化为字符串，格式为 年-月-日-时-分-秒
pub fn get_current_time_format_string() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d-%H-%M-%S").to_string()
}
