use diesel::prelude::*;

#[derive(Queryable,Selectable, Debug)]
#[diesel(table_name=crate::schema::products)]
pub struct Product{
    pub id:i32,
    pub name:String,
    pub price:f64,
    pub category:String,
}

#[derive(Insertable)]
#[diesel(table_name=crate::schema::products)]
pub struct ProductInsert{
    pub name:String,
    pub price:f64,
    pub category:String,
}