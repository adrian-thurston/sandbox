

macro_rules! aliased_map {
    ( $($x:tt)* ) => ( maplit::hashmap!( $( $x )* ) );
}

fn main() {
    let m = aliased_map!{
        "foo" => "bar",
        "baz" => "bam",
    };
    println!("hello world: {:?}", m );
}
