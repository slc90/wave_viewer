use singlyton::SingletonUninit;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::state::{background_result::BackgroundResult, ui_command::UiCommand};

/// UI线程的sender和receiver
pub static SEND_TO_BACKGROUND: SingletonUninit<Sender<UiCommand>> = SingletonUninit::uninit();
pub static RECEIVE_BACKGROUND_MESSAGE: SingletonUninit<Receiver<BackgroundResult>> =
    SingletonUninit::uninit();

/// 因为egui中不能直接使用异步,所以封装在一个tokio::spawn中
/// 这个函数只能在Ui线程中使用
/// # Arguments
///
/// - `command` (`UiCommand`) - Ui向后台发送的命令
///
/// # Returns
///
/// - `()` - 不需要关注返回值.
pub(super) fn send_to_background(command: UiCommand) -> () {
    tokio::spawn(async move {
        let _ = SEND_TO_BACKGROUND.get().send(command).await;
    });
}
