use std::str::FromStr;

use chrono::Local;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

use crate::configs::Configs;

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

pub async fn init(cfg: &Configs) -> WorkerGuard {
    let level = Level::from_str(&cfg.log.level).expect("Invalid log level");
    let (non_blocking, guard) = if cfg.server.env == "prod" {
        // 使用tracing_appender，指定日志的输出目标位置
        // 参考: https://docs.rs/tracing-appender/latest/tracing_appender/index.html
        let (non_blocking, guard) = tracing_appender::non_blocking(
            tracing_appender::rolling::daily(&cfg.log.path, &cfg.log.filename),
        );
        (non_blocking, guard)
    } else {
        let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());
        (non_blocking, guard)
    };

    // 初始化并设置日志格式(定制和筛选日志)
    tracing_subscriber::fmt()
        .with_max_level(level) //只有Debug 模式下才能打印sea-orm的完整sql日志
        .with_file(true)
        .with_line_number(true) // 写入标准输出
        .with_ansi(true) // 关掉ansi的颜色输出功能
        .with_timer(LocalTimer)
        .with_writer(non_blocking)
        // .json()
        // .flatten_event(true)
        .init(); // 初始化并将SubScriber设置为全局SubScriber

    guard
}
