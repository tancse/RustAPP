use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

mod schema;
mod models;
use models::{Product,ProductInsert};

use schema::products::dsl::*;

fn get_connection() -> SqliteConnection{
    let db_url = "store.db";
    SqliteConnection::establish(db_url)
    .except("Failed to connect!")
}

fn creat_table(){
   let mut con= get_connection();
   diesel::sql_query(
        "
        CREATE TABLE IF NOT EXISTS products(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        price REAL NOT NULL,
        category TEXT NOT NULL
        "
   ).execute(&con)
   .expect("Failed to create table");

   println!("Products Table Created Succesfully!")
}

fn main() {
    creat_table();
}
