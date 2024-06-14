use std::time::Duration;
use tokio::runtime::{Builder, Runtime};


fn main() {
    let runtime = new_multi_thread().unwrap();
    runtime.block_on(async {});
}


/// 新建多线程运行时
///
/// # 返回
/// - 成功: 多线程运行时
/// - 错误: 返回对应的错误类型
#[allow(dead_code)]
fn new_multi_thread() -> std::io::Result<Runtime> {
    builder(Builder::new_multi_thread().worker_threads(8))
}


/// 使用当前线程新建运行时
///
/// # 返回
/// - 成功: 多线程运行时
/// - 错误: 返回对应的错误类型
#[allow(dead_code)]
fn new_current_thread() -> std::io::Result<Runtime> {
    builder(&mut Builder::new_current_thread())
}


fn builder(builder: &mut Builder) -> std::io::Result<Runtime> {
    builder.enable_all()
        .max_io_events_per_tick(1024)
        .thread_keep_alive(Duration::from_secs(60))
        .build()
}
