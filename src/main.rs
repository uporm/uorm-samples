use crate::config::AppConfig;
use crate::web::server::WebServer;
use rivus_logger::LoggerConfig;
use uorm::driver_manager::U;
use uorm::mapper_assets;
use uorm::udbc::PoolOptions;
use uorm::udbc::mysql::pool::MysqlDriver;

mod business;
mod config;
mod models;
mod routes;
mod utils;
mod web;

mapper_assets!["resources/mappers"];
// 初始化翻译文件
rust_i18n::i18n!("resources/locales", fallback = "zh");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conf = AppConfig::load();

    // 1. 初始化日志
    let _guard = LoggerConfig::new()
        .enable_console(conf.logger.console)
        .level(conf.logger.level)
        .init();

    // 2. 初始化数据库
    let db_conf = conf.database;
    let mut driver = MysqlDriver::new(db_conf.url);
    if let Some(opt) = db_conf.option {
        driver = driver.options(PoolOptions {
            max_open_conns: opt.max_open_conns,
            max_idle_conns: opt.max_idle_conns,
            max_lifetime: opt.max_lifetime,
            timeout: opt.timeout,
        });
    }

    let driver = driver.build()?;
    U.register(driver)?;

    WebServer::new(&conf.server)
        .mount(routes::router())
        .layer_i18n()
        .start()
        .await?;

    Ok(())
}
