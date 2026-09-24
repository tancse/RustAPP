diesel::table!{
    products(id){
        id -> Integer,
        name -> Text,
        price -> Double,
        category -> Text,
    }
}