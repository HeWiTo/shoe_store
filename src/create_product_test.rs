use diesel::result::Error;
use ::shoe_store::establish_connection_test;

#[test]
fn create_product_test() {
    let connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|| {
        create_product(NewCompleteProduct {
            product: NewProduct {
                name: "boots".to_string(),
                cost: 13.23,
                active: true
            },
            variants: vec![
                NewVariantValue {
                    variant: NewVariant {
                        name: "size".to_string()
                    },
                    values: vec![
                        Some(12.to_string()),
                        Some(14.to_string()),
                        Some(16.to_string()),
                        Some(18.to_string())
                    ]
                }
            ]
        }, &connection).unwrap();

        assert_eq!(
            serde_json::to_string(&list_products(&connection).unwrap()).unwrap(),
            serde_json::to_string(&vec![
                (
                    Product {
                        id: 1,
                        name: "boots".to_string(),
                        cost: 13.23,
                        active: true
                    },
                    vec![
                        (
                            Some(12.to_string()),
                            "size".to_string()
                        ),
                        (
                            Some(14.to_string()),
                            "size".to_string()
                        ),
                        (
                            Some(16.to_string()),
                            "size".to_string()
                        ),
                        (
                            Some(18.to_string()),
                            "size".to_string()
                        )
                    ]
                ),
            ]).unwrap()
        );

        Ok(())
    });
}
