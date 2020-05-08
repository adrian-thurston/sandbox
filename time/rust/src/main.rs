use chrono::prelude::*;

fn main() {
	let lit = "2018-11-29T09:00:00Z";
    println!("{}", DateTime::parse_from_rfc3339(lit).unwrap() );
}
