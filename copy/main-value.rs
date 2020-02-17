pub struct Struct {
    i: i32,
    j: i32,
}

impl Copy for Struct {}
impl Clone for Struct
{
    fn clone(&self) -> Struct {
        println!("Struct::clone");
        Struct { i: self.i, j: self.j }
    }
}

fn modify( mut s: Struct ) -> Struct
{
    s.i = 117;
    return s;
}

pub fn work() -> Struct {
    let mut s = Struct{ i: 1, j: 2 };
    s = modify( s );
    s
}

pub fn main() {
    let s = work();
    println!("{}", s.i);
}
