
use crate::models::*;
use deadpool_postgres::Client;
use std::io::Error;
use tokio_pg_mapper::FromTokioPostgresRow;
pub async fn get_todos(client: &Client) -> Result<Vec<ToDoList>, Error> {

    let statement = client
        .prepare("select * from todo_list")
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