mod db_operations{

    pub fn insert(){
        println!("Insert Code");
    }

    pub fn update(){
        println!("update Code");
    }
}

fn main() {
    db_operations::insert();
    db_operations::update();
}