// use mysql_async::prelude::*;
// use mysql_async::{IsolationLevel, TxOpts};
//
// #[tokio::main]
// async fn main() {
//     // let sql = "begin;
//     // update marketing_counter
//     //     set `count` = `count` + 1
//     //     where code='0000005d95624e998f12e3fad6807f29';
//     //     select `count` from marketing_counter where code = '0000005d95624e998f12e3fad6807f29';
//     //     commit;";
//
//     let sql = "update marketing_counter
//         set `count` = `count` + 1
//         where code='0000005d95624e998f12e3fad6807f29';
//     select `count` from marketing_counter where code = '0000005d95624e998f12e3fad6807f29';";
//
//     let url = "mysql://marketing_counter:NdyCKO18ghdkFEr1@10.177.9.49:28282/marketing_counter_db";
//
//     let pool = mysql_async::Pool::new(url);
//     let mut conn = pool.get_conn().await.unwrap();
//
//     // let mut opts = TxOpts::new();
//     // opts.with_isolation_level(IsolationLevel::RepeatableRead);
//     //
//     // let mut transaction = conn.start_transaction(opts).await.unwrap();
//     // transaction.prep(sql).await;
//     conn.query(sql)
//         .await
//         .map(|vec: Vec<String>| println!("result={:?}", vec));
//     // transaction.commit().await;
// }

use mysql_async::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let payments = vec![
        Payment {
            customer_id: 1,
            amount: 2,
            account_name: None,
        },
        Payment {
            customer_id: 3,
            amount: 4,
            account_name: Some("foo".into()),
        },
        Payment {
            customer_id: 5,
            amount: 6,
            account_name: None,
        },
        Payment {
            customer_id: 7,
            amount: 8,
            account_name: None,
        },
        Payment {
            customer_id: 9,
            amount: 10,
            account_name: Some("bar".into()),
        },
    ];

    let database_url = /* ... */

    let pool = mysql_async::Pool::new(database_url);
    let mut conn = pool.get_conn().await?;

    // Create temporary table
    conn.query_drop(
        r"CREATE TEMPORARY TABLE payment (
            customer_id int not null,
            amount int not null,
            account_name text
        )",
    )
    .await?;

    // Save payments
    let params = payments.clone().into_iter().map(|payment| {
        params! {
            "customer_id" => payment.customer_id,
            "amount" => payment.amount,
            "account_name" => payment.account_name,
        }
    });

    conn.exec_batch(
        r"INSERT INTO payment (customer_id, amount, account_name)
          VALUES (:customer_id, :amount, :account_name)",
        params,
    )
    .await?;

    // Load payments from database. Type inference will work here.
    let loaded_payments = conn
        .exec_map(
            "SELECT customer_id, amount, account_name FROM payment",
            (),
            |(customer_id, amount, account_name)| Payment {
                customer_id,
                amount,
                account_name,
            },
        )
        .await?;

    // Dropped connection will go to the pool
    conn;

    // Pool must be disconnected explicitly because
    // it's an asynchronous operation.
    pool.disconnect().await?;

    assert_eq!(loaded_payments, payments);

    // the async fn returns Result, so
    Ok(())
}
