use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;


#[derive(Serialize)]
pub struct Status { 
    pub status: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "todo_item")]
pub struct ToDoItem { 
    pub id: i32, 
    pub title: String, 
    pub checked: bool, 
    pub list_id: i32
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "todo_list")]
pub struct ToDoList { 
    pub id: i32, 
    pub title: String
}