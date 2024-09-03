use ::entity::post::{self, CategoryEnum, Entity as Post};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, EntityTrait, ModelTrait,
    QueryFilter, Set,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // connnect database
    let db: DatabaseConnection =
        Database::connect("postgres://postgres:postgres@localhost:5433/test_db")
            .await
            .expect("Failed to connect to db");
    //insert
    let post_model: post::ActiveModel = post::ActiveModel {
        title: Set("Title".to_string()),
        content: Set("Content".to_string()),
        category: Set(CategoryEnum::NixOS),
        ..Default::default()
    };
    let post: sea_orm::InsertResult<post::ActiveModel> =
        Post::insert(post_model).exec(&db).await.unwrap();
    println!("Inserted: {:?}", post);
    //find
    let post_data: Option<post::Model> = Post::find()
        .filter(post::Column::Category.eq(CategoryEnum::NixOS))
        .one(&db)
        .await
        .unwrap();
    println!("Found post data\n{:#?}", post_data);
    //update
    match post_data {
        Some(post_data) => {
            let mut update_post_data: post::ActiveModel = post_data.into();
            update_post_data.title = Set("Updated Title".to_string());
            update_post_data.content = Set("Updated Content".to_string());
            update_post_data.category = Set(CategoryEnum::Windows);
            let updated_post: post::Model = update_post_data.update(&db).await.unwrap();
            println!("Updated post data\n{:#?}", updated_post);
        }
        None => {
            println!("No post data found");
        }
    }
    //delete
    let delete_post_data: Option<post::Model> = Post::find()
        .filter(post::Column::Category.eq(CategoryEnum::Windows))
        .one(&db)
        .await
        .unwrap();
    match delete_post_data {
        Some(delete_post_data) => {
            let deleted_post: sea_orm::DeleteResult = delete_post_data.delete(&db).await.unwrap();
            println!("Deleted post data\n{:?}", deleted_post);
        }
        None => {
            println!("No post data found");
        }
    }
}
