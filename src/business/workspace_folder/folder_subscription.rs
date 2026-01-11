use crate::business::workspace_folder::folder_dao::{Folder, FolderDao};
use crate::core::subscription::WorkspaceSubscription;
use anyhow::anyhow;
use async_trait::async_trait;

struct FolderSubscription;

#[async_trait]
impl WorkspaceSubscription for FolderSubscription {
    fn topic(&self) -> &'static str {
        "workspace_folder"
    }

    async fn consume(&self, tenant_id: u64, workspace_id: u64) -> anyhow::Result<()> {
        let mut folder = Folder::new(tenant_id, workspace_id);
        folder.folder_type = 1;
        folder.parent_id = 0;
        folder.name = "默认目录".to_string();
        folder.seq = 1;

        if let Err(e) = FolderDao::insert(&folder).await {
            return Err(anyhow!("创建默认目录失败: {:?}", e));
        }
        Ok(())
    }
}

// 将业务逻辑“发布”到全局名单
inventory::submit! { &FolderSubscription as &dyn WorkspaceSubscription }
