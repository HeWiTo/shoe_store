use ::shoe_store::models::NewProduct;
use diesel::sqlite::SqliteConnection;
use diesel::result::Error;
use diesel::RunQueryDsl;

fn create_product(new_product: NewProduct, conn: &SqliteConnection) -> Result<usize, Error> {
    use ::shoe_store::schema::products::dsl::*;
    diesel::insert_into(products)
        .value(new_product)
        .execute(conn)
}
