use tracing::Level;

pub fn init_logger(level: Level) {
    tracing_subscriber::fmt()
        .with_max_level(level) //只有Debug 模式下才能打印sea-orm的完整sql日志
        .with_writer(std::io::stdout)
        // .with_test_writer()
        .with_target(false)
        .compact()
        .init();
}
