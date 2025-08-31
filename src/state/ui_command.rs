/// Ui向后台发送的所有命令
///
/// # Variants
///
/// - `TestCommand` - 测试用命令
#[derive(Debug)]
pub enum UiCommand {
    TestCommand,
}
