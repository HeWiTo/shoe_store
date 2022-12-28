use diesel::Connection;
use ::shoe_store::establish_connection_test;

#[test]
fn create_product_test() {
    let connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|| {
        let result = 
            create_product(NewProduct {
                name: "boots".to_string(),
                cost: 13.23,
                active: true
            }, &connection);
        assert_eq!(Ok(1), results);

        Ok(())
    });
}
