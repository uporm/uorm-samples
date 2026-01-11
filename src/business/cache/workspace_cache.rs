use moka::future::Cache;
use std::sync::LazyLock;
use std::time::Duration;

// 工作空间缓存： user_id -> workspace_id
pub static WORKSPACE_CACHE: LazyLock<Cache<u64, u64>> = LazyLock::new(|| {
    Cache::builder()
        .max_capacity(100_000)
        .time_to_live(Duration::from_secs(24_60_60))
        .build()
});


pub async fn get_workspace_id(user_id: u64) -> Option<u64> {
    WORKSPACE_CACHE.get(&user_id).await
}

pub async fn switch_workspace(user_id: u64, workspace_id: u64) {
    WORKSPACE_CACHE.insert(user_id, workspace_id).await;
}
