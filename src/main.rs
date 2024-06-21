
mod mysql_db {
    include!("./db/mysql_db.rs");
}

fn main() {
    mysql_db::test_db();
    println!("Hello, world!");

}
