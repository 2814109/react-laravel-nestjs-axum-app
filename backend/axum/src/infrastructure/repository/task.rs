use crate::entity::prelude::Tasks;
use crate::entity::tasks;
use axum::response;
use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DatabaseConnection, EntityTrait, JsonValue};
use serde::{Serialize, Deserialize};

use axum::{
    extract::Path,
};
use axum::{
    Json,
  
};
// use std::collections::HashMap;


#[derive(Serialize)]
pub struct Task {
    id: i32,
}

#[derive(Serialize)]
pub struct TaskItem {
    id: i32,
    title: String,
    is_closed: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct TaskList {
    pub task_list: Vec<JsonValue>,
}


#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
}

pub async fn create_one(title: &str) {
    println!("{:?}", title);
    let utc: DateTime<Utc> = Utc::now();
    // entity を使ってinsert処理を実行
    let new_task = tasks::ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set(title.to_owned()),
        is_closed: ActiveValue::Set(false),
        created_at: ActiveValue::Set(utc.naive_utc()),
        updated_at: ActiveValue::Set(utc.naive_utc()),
    };
    let db: DatabaseConnection =
        Database::connect("postgresql://postgres:postgres@localhost:5432/postgres".to_string())
            .await
            .expect("Database connection failed");

    let result = new_task.insert(&db).await;
    println!("{:?}", result);
}

pub async fn get_task_by_id() -> response::Json<Task> {
    // let urlPathParameterId = id.to_string();
    
    // let intId =  as i32;
    let db: DatabaseConnection =
        Database::connect("postgresql://postgres:postgres@localhost:5432/postgres".to_string())
            .await
            .expect("Database connection failed");

    let result = Tasks::find_by_id(5).one(&db).await;

    let get_id: i32 = match result {
        Ok(Some(tasks)) => tasks.id,
        Err(_) => todo!(),
        Ok(None) => todo!(),
    };
    return response::Json(Task { id: get_id });
}


pub async fn get_all_task() -> response::Json<TaskList> {
    let db: DatabaseConnection =
        Database::connect("postgresql://postgres:postgres@localhost:5432/postgres".to_string())
            .await
            .expect("Database connection failed");

    let results = Tasks::find().into_json().all(&db).await;
    // let tasks_data: Vec<TaskItem> = match results {
    //     Ok(tasks) => tasks,
    //     Err(_) => todo!(),
    //     // Ok(None) => todo!(),
    // };
    // println!("{:?}", tasks_data);

    // let results_data =    tasks_data.into_iter().map(|argument| argument.id);

    // let results_data = tasks_data.into_iter().map(|argument| argument.id);
    // println!("{:?}", results_data);

    // for value in &tasks_data {
    //     println!("{:?}", value);
    // }
    // return response::Json::<Vec<Tasks>>(&tasks_data.unwrap());

    return response::Json(TaskList { task_list: results.unwrap() });
}


pub async fn get_id(Path(id) :Path<String>) -> response::Json<Task> {

 println!("{:?}", id);

 let int_id:i32 = id.parse().unwrap();
 
        let db: DatabaseConnection =
        Database::connect("postgresql://postgres:postgres@localhost:5432/postgres".to_string())
            .await
            .expect("Database connection failed");

    let result = Tasks::find_by_id(int_id).one(&db).await;

    let get_id: i32 = match result {
        Ok(Some(tasks)) => tasks.id,
        Err(_) => todo!(),
        Ok(None) => todo!(),
    };

     println!("get id {:?}",get_id);

    return response::Json(Task { id: get_id });
    
}


pub async fn create_one_task (Json(payload): Json<CreateTask>) {
    let body = payload;

     println!("request body {:?}",body.title);

    let utc: DateTime<Utc> = Utc::now();
    // entity を使ってinsert処理を実行
    let new_task = tasks::ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set(body.title.to_owned()),
        is_closed: ActiveValue::Set(false),
        created_at: ActiveValue::Set(utc.naive_utc()),
        updated_at: ActiveValue::Set(utc.naive_utc()),
    };
    let db: DatabaseConnection =
        Database::connect("postgresql://postgres:postgres@localhost:5432/postgres".to_string())
            .await
            .expect("Database connection failed");

    let result = new_task.insert(&db).await;
    println!("{:?}", result);
}