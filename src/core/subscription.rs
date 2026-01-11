use async_trait::async_trait;

#[async_trait]
pub trait WorkspaceSubscription: Send + Sync {
    // 订阅的主题或任务 ID
    fn topic(&self) -> &'static str;

    // 消费逻辑
    async fn consume(&self, tenant_id: u64, workspace_id: u64) -> anyhow::Result<()>;
}

// 建立全局订阅名单（静态注册）
inventory::collect!(&'static dyn WorkspaceSubscription);