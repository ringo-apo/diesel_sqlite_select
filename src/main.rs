extern crate diesel;
extern crate sample_proj;

use self::diesel::prelude::*;
use self::sample_proj::*;
use self::models::*;

fn main() {
    use sample_proj::schema::memos::dsl::*;

    let connection = establish_connection();
    let results = memos
        .limit(5)
        .load::<Memo>(&connection)
        .expect("Error loading memos");

    println!("Displaying {} memos", results.len());
    println!("id | comment");
    for memo in results {
        println!("{} | {}", memo.id, memo.comment);
    }
}

