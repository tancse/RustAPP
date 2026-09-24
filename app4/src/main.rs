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
        name: "iphone".to_string(),
        price: 250000.00,
        category: "mobile".to_string(),
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

fn update(){
    let mut con = get_connection();
    let product_id = 1;

    diesel::update(products.filter(id.eq(product_id))).set((
        name.eq("Dell 3525"),
        price.eq(75000.00),
        category.eq("Laptop"),
    )).execute(&mut con).expect("Failed to update");

    println!("Records Updated Successfully");
}

fn delete(){
    let mut con = get_connection();
    let product_id = 2;

    diesel::delete(products.filter(id.eq(product_id))).execute(&mut con).expect("Failed to delete");

    println!("Record delete successfully");
}

fn main() {
    //create_table();
    //insert();
    get_data();
    delete();
    get_data();
}
