extern crate mysql;
extern crate r2d2;
extern crate r2d2_mysql;

use std::sync::Arc;
use std::thread;
// use mysql::{Opts,OptsBuilder};
// use mysql::prelude::Queryable;
use r2d2_mysql::mysql::prelude::Queryable;
use r2d2_mysql::mysql::{Opts, OptsBuilder};
use r2d2_mysql::MysqlConnectionManager;

fn main() {
    let sql = "update marketing_counter
        set `count` = `count` + 1
        where code='0000005d95624e998f12e3fad6807f29'";

    let url = "mysql://marketing_counter:NdyCKO18ghdkFEr1@10.177.9.49:28282/marketing_counter_db";
    let opts = Opts::from_url(url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let manager = MysqlConnectionManager::new(builder);
    let pool = Arc::new(r2d2::Pool::builder().max_size(4).build(manager).unwrap());

    let mut tasks = vec![];

    for _ in 0..3 {
        let pool = pool.clone();
        let th = thread::spawn(move || {
            let mut conn = pool
                .get()
                .map_err(|err| {
                    println!(
                        "get connection from pool error in line:{} ! error: {:?}",
                        line!(),
                        err
                    )
                })
                .unwrap();
            let _ = conn
                .query(sql)
                .map(|vec: Vec<String>| println!("result={:?}", vec))
                .map_err(|err| {
                    println!("execute query error in line:{} ! error: {:?}", line!(), err)
                });
        });
        tasks.push(th);
        // std::thread::sleep(std::time::Duration::from_secs(1));
    }

    for th in tasks {
        let _ = th.join();
    }
}
