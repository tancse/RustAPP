use rusqlite::Connection;

pub fn connect_db() -> Connection {
    let conn = Connection::open("store.db").unwrap();
    println!("Connected to DB!");
    conn

}

pub fn create_table(connection: &Connection){
    connection.execute(
        "CREATE TABLE IF NOT EXISTS products(
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            price REAL NOT NULL
        )",
    [],).unwrap();

    println!("Table Created Successfully");
}

pub fn insert(connection: &Connection, name:&str, price: f64){
    connection.execute(
        "INSERT INTO products(name,price) VALUES(?1,?2)", 
        (name,price),
    ).unwrap();

    println!("Data Inserted Successfully");
}

pub fn get_all(connection: &Connection){
    let mut statement = connection.prepare("SELECT * FROM products").unwrap();

    let products = statement.query_map([], |row|{
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, f64>(2)?,

        ))
    }).unwrap();

    for prod in products{
        let (id,name,price) = prod.unwrap();

        println!("Id:{}\tName: {}\tPrice:{}", id,name,price);
    }
}

pub fn update(connection: &Connection, id:i32, name:&str, price:f64){
    let rowcount = connection.execute(
        "UPDATE products set name=?1,price=?2 WHERE id=?3",
        (name,price,id)
    ).unwrap();

    println!("{} Rows udated Successfully!", rowcount);
}

pub fn delete(connection: &Connection, id:i32){
    let rowcount = connection.execute(
        "DELETE FROM products WHERE id=?1",
        [id]
    ).unwrap();

    println!("{} Rows deleted Successfully!", rowcount);
}