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

fn modify( s: &mut Struct )
{
    s.i = 116;
}

pub fn work() -> Struct {
    let mut s = Struct{ i: 1, j: 2 };
    modify( &mut s );
    s
}

pub fn main() {
    let s = work();
    println!("{}", s.i);
}
