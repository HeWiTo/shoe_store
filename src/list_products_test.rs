use diesel::result::Error;
use diesel::Connection;
use ::shoe_store::establish_connection_test;

#[test]
fn test_list_products() {
    let connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|| {
        create_product(NewProduct {
            name: "boots".to_string(),
            cost: 13.23,
            active: true
        }, &connection);
        create_product(NewProduct {
            name: "high heels".to_string(),
            cost: 20.99,
            active: true
        }, &connection);
        create_product(NewProduct {
            name: "running shoes".to_string(),
            cost: 10.99,
            active: true
        }, &connection);

        assert_eq!(serde_json::to_string(&list_products(&connection).unwrap(),
            serde_json::to_string(&vec![
                Product {
                    id: 1,
                    name: "boots".to_string(),
                    cost: 13.23,
                    active: true
                },
                Product {
                    id: 2,
                    name: "high heels".to_string(),
                    cost: 20.99,
                    active: true
                },
                Product {
                    id: 3,
                    name: "running shoes".to_string(),
                    cost: 10.99,
                    active: true
                }
            ]).unwrap());

        Ok(())
    });
}
