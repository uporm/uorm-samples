use tracing::info;
use crate::core::subscription::WorkspaceSubscription;

pub struct WorkspaceConsumer;

impl WorkspaceConsumer {
    pub async fn start_dispatching(tenant_id: u64, workspace_id: u64) -> anyhow::Result<()> {

        // 获取所有已注册的订阅
        let subscriptions = inventory::iter::<&dyn WorkspaceSubscription>;
        for sub in subscriptions {
            println!("监听到主题: [{}]，准备消费...", sub.topic());
            sub.consume(tenant_id, workspace_id).await?;
        }

        info!("工作空间 [{}] 所有订阅已消费完成", workspace_id);
        Ok(())
    }
}