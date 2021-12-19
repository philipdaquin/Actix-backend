
use crate::models::*;
use deadpool_postgres::Client;
use std::io::{Error,  ErrorKind};
use tokio_pg_mapper::FromTokioPostgresRow;


pub async fn get_todos(client: &Client) -> Result<Vec<ToDoList>, Error> {

    let statement = client
        .prepare("select * from todo_list order by id desc")
        .await
        .unwrap();

    let todos = client.query(&statement, &[])
        .await
        .expect("Error Fetching To do list")
        .iter()
        .map(|row| ToDoList::from_row_ref(row).unwrap())
        .collect::<Vec<ToDoList>>();
    Ok(todos)
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<ToDoItem>, Error> { 

    let statement = client 
        .prepare("select * from todo_item where list_id = $1 order by id")
        .await
        .unwrap();
    
    let items = client.query(&statement, &[&list_id])
        .await
        .expect("Failed to load user items")
        .iter()
        .map(|row| ToDoItem::from_row_ref(row).unwrap())
        .collect::<Vec<ToDoItem>>();

    Ok(items)
}

pub async fn create_items(client: &Client, title: String ) -> Result<ToDoList, Error> { 
    let statement = client 
        .prepare("insert into todo_list (title) values ($1) returning id, title")
        .await
        .unwrap();
    let items = client
        .query(&statement, &[&title])
        .await 
        .expect("Could not Create a todo list")
        .iter()
        .map(|row| ToDoList::from_row_ref(row).unwrap())
        .collect::<Vec<ToDoList>>()
        .pop()
        .ok_or(Error::new(ErrorKind::Other, "Error creating to do list"));
    items
      

}
pub async fn check_item(client: &Client, list_id: i32, item_id: i32) -> Result<(), Error> { 
    
    let statement = client  
        .prepare("update todo_item set checked = true where list_id = $1 and id = $2  and checked = false ")
        .await
        .unwrap();
    let result = client
        .execute(&statement, &[&list_id, &item_id])
        .await
        .expect("Error checking todo item");
    
    match result { 
        ref updated if *updated == 1 => Ok(()),
        _ => Err(Error::new(ErrorKind::Other, "Failed to check  list"))
    }

}

