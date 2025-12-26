use rivus_axum::i18n::i18n::t;
use rivus_axum::WebServer;
use rivus_axum_macro::i18n_assets;
use crate::config::AppConfig;
use rivus_logger::LoggerConfig;
use uorm::driver_manager::U;
use tracing::info;
use uorm::udbc::mysql::pool::MysqlDriver;
use uorm::udbc::PoolOptions;

mod business;
mod config;
mod routes;

// mapper_assets!["../resources/mappers/*.xml"];
i18n_assets!["resources/locales"];

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
        

    let msg_en = t("en", "500", &[
    ]);
    info!("I18n test: {}", msg_en);

    WebServer::new(&conf.server)
        .layer_i18n()
        .mount(routes::app_router())
        .start()
        .await?;

    Ok(())
}
