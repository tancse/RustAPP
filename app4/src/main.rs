use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

mod schema;
mod models;
use models::{Product,ProductInsert};

use schema::products::dsl::*;

fn get_connection() -> SqliteConnection{
    let db_url = "store.db";
    SqliteConnection::establish(db_url)
    .expect("Failed to connect!")
}

fn create_table(){
   let mut con= get_connection();
   diesel::sql_query(
        "
        CREATE TABLE IF NOT EXISTS products(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        price REAL NOT NULL,
        category TEXT NOT NULL
        )    
        "
   ).execute(&mut con)
   .expect("Failed to create table");

   println!("Products Table Created Succesfully!")
}
fn insert() {
    let prod = ProductInsert{
        name: "Dell 3525".to_string(),
        price: 50000.00,
        category: "Laptop".to_string(),
    };

    let mut con = get_connection();

    diesel::insert_into(products).values(prod).execute(&mut con).expect("Failed to Insert!");

    println!("Record Inserted Successfully!")
}

fn get_data(){
    let mut con = get_connection();

    let product_list:Vec<Product> = products.select(Product::as_select()).load(&mut con).expect("Failed to get records");

    for pr in product_list{
        println!("Id:{}\tName:{}\tPrice:{}\tCategory:{}",pr.id,pr.name,pr.price,pr.category);
    }
}


fn main() {
    //create_table();
    //insert();
    get_data();
}
