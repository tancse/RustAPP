mod dboperations;

use dboperations as db;

fn main() {
    let conn = db::connect_db();
    //db::create_table(&conn);
    //db::insert(&conn, "Iphone", 45000.00 );
    // db::get_all(&conn);
    // db::update(&conn, 1, "Samsung", 65000.00);
    // db::update(&conn, 3, "G-Pixel", 89000.00);
    // db::update(&conn, 4, "Redmi 13", 12000.00);
    db::delete(&conn, 4);
    db::get_all(&conn);
}