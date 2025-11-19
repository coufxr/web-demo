use std::str::FromStr;

use super::configs::Configs;
use chrono::Local;
use tracing::Level;
use tracing_appender::{non_blocking, non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{
    EnvFilter,
    fmt::{format::Writer, time::FormatTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

/// 自定义 Local 时间格式
struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

pub async fn init(cfg: &Configs) -> WorkerGuard {
    // 设置日志等级
    let level = Level::from_str(&cfg.log.level).unwrap_or(Level::INFO);

    // 生产环境 → 写入文件
    // 开发环境 → 输出到 stdout
    let (writer, guard) = if cfg.app.env == "prod" {
        // 使用tracing_appender，指定日志的输出目标位置
        // 参考: https://docs.rs/tracing-appender/latest/tracing_appender/index.html
        let file_appender = rolling::daily(&cfg.log.path, &cfg.log.filename);
        non_blocking(file_appender)
    } else {
        non_blocking(std::io::stdout())
    };

    // EnvFilter
    let filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(level.as_str()));

    // fmt layer（官方推荐）
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_writer(writer)
        .with_ansi(true) // 显示ansi的颜色输出
        .with_timer(LocalTimer) // 日期格式
        .with_file(false) // 显示源代码文件路径
        .with_line_number(true) // 显示源代码行号
        .with_thread_ids(true) // 显示记录事件的线程ID
        .with_target(true) // 显示事件的目标（模块路径）
        .compact() // 使用更紧凑、缩写的日志格式
        // .pretty() // 漂亮的多行日志,用于本地开发和调试
        // .json() // json格式
        // .flatten_event(true) // 为json展平事件元数据
        ;

    // registry + layer（官方示例风格）
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();

    guard
}
