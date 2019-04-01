extern crate batch_gcd;
extern crate mysql;
extern crate rayon;
extern crate rug;

use batch_gcd::batch_gcd;
use rayon::prelude::*;
use rug::Integer;
use std::time::Instant;

type Row = (String, String);

fn login_db(username: &str, passwd: &str, hostname: &str, db_name: &str) -> mysql::Pool {
    mysql::Pool::new(&format!(
        "mysql://{}:{}@{}/{}",
        username, passwd, hostname, db_name
    ))
    .unwrap()
}

fn main() {
    let now = Instant::now();
    let container = login_db("iosmanthus", "????", "localhost", "iosmanthus_db")
        .prep_exec("SELECT name,modulus from cert_table limit 50000", ())
        .map(|result| {
            result
                .map(|x| -> Row { mysql::from_row(x.unwrap()) })
                .collect::<Vec<_>>()
        })
        .unwrap();

    let names = container
        .par_iter()
        .map(|(name, _)| name)
        .collect::<Vec<_>>();

    let moduluses = container
        .par_iter()
        .map(|(_, modulus)| Integer::from(Integer::parse(&modulus).unwrap()))
        .collect::<Vec<_>>();

    println!(
        "{:#?}",
        names
            .par_iter()
            .zip(batch_gcd(&moduluses).par_iter())
            .zip(moduluses.par_iter())
            .filter(|((_, result), modulus)| **result != Integer::from(1) && modulus != result)
            .collect::<Vec<_>>()
    );
    println!("{}", now.elapsed().as_secs());
}
