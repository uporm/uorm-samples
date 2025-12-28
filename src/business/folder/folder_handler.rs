use crate::web::r::R;
use crate::business::folder::folder_dao::FolderDao;
use crate::business::folder::folder_model::{CreateFolder, Folder, MoveFolder, UpdateFolder};
use axum::Json;
use axum::extract::Path;
use tracing::debug;
use validator::Validate;
use crate::r;

pub async fn get_folder_tree(Path(r#type): Path<i32>) -> R<Vec<Folder>> {
    debug!("type: {}", r#type);

    let folder1 = Folder {
        id: 1,
        name: Some("Root Folder 1".to_string()),
        description: Some("Description for Root 1".to_string()),
        children: vec![
            Folder {
                id: 11,
                name: Some("Child Folder 1.1".to_string()),
                description: Some("Description for Child 1.1".to_string()),
                children: vec![],
            },
            Folder {
                id: 12,
                name: Some("Child Folder 1.2".to_string()),
                description: Some("Description for Child 1.2".to_string()),
                children: vec![],
            },
        ],
    };

    let folder2 = Folder {
        id: 2,
        name: Some("Root Folder 2".to_string()),
        description: Some("Description for Root 2".to_string()),
        children: vec![],
    };

    R::ok(vec![folder1, folder2])
}

pub async fn create_folder(Path(r#type): Path<i32>, Json(mut folder): Json<CreateFolder>) -> R<i32> {
    r!(folder.validate());
    let s = FolderDao::insert(&mut folder).await;
    debug!("type: {:?}", folder.ids);
    R::from(s)
}

pub async fn update_folder(
    Path((r#type, _id)): Path<(i32, i32)>,
    Json(folder): Json<UpdateFolder>,
) -> R<()> {
    r!(folder.validate());
    debug!("type: {}", r#type);
    debug!("update_folder: {:?}", folder);
    R::void()
}

pub async fn delete_folder(Path((r#type, id)): Path<(i32, i32)>) -> R<()> {
    debug!("type: {}", r#type);
    debug!("id: {}", id);
    R::void()
}

pub async fn move_folder(
    Path((r#type, _id)): Path<(i32, i32)>,
    Json(folder): Json<MoveFolder>,
) -> R<()> {
    r!(folder.validate());
    debug!("type: {}", r#type);
    debug!("move_folder: {:?}", folder);
    R::void()
}
