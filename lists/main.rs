

struct Token
{
    i: i32,
    j: i32,

    next: Option<Box<Token>>,
}


fn make() -> Option<Box<Token>> {
    let mut t1 = Token{ i: 1, j: 2, next: None };
    let mut t2 = Token{ i: 3, j: 4, next: None };
    let mut t3 = Token{ i: 5, j: 6, next: None };
    let mut t4 = Token{ i: 7, j: 8, next: None };

    let mut collect = None;

    t1.next = collect.take();
    collect = Some(Box::new(t1));

    t2.next = collect.take();
    collect = Some(Box::new(t2));

    t3.next = collect.take();
    collect = Some(Box::new(t3));

    t4.next = collect.take();
    collect = Some(Box::new(t4));

    return collect;
}

fn reverse( mut head: Option<Box<Token>> ) -> Option<Box<Token>> {
    let mut reversed = None;
    while let Some(mut boxed_head) = head {
        let next = (*boxed_head).next.take();
        (*boxed_head).next = reversed.take();
        reversed = Some(boxed_head);
        head = next;
    }
    return reversed;
}

fn print( mut r: Option<Box<Token>> ) -> () {
    while let Some(b) = r {
        println!("{} {}", (*b).i, (*b).j);
        r = (*b).next;
    }
}

fn main() {
    let t = make();
    let r = reverse( t );
    print( r );
}
