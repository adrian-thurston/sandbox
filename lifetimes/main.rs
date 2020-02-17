struct IntStruct<'a> {
    ref_to_int: &'a i32,
}

// With this declaration we separate the lifetime of the whole struct from the
// lifetime of the reference it contains. It is saying that they are not
// necessarily bound together. This allows answer to live past the block in
// which scoped_struct is declared.
fn some_process<'a, 'b, 'c>(a: &'a i32, a_struct: &'b IntStruct<'a>) -> &'a i32 {
    if *a == 101 {
        a_struct.ref_to_int
    } else {
        a
    }
}

pub fn main() {
    let a: i32 = 101;
    let int_struct: IntStruct = IntStruct { ref_to_int: &a };
    let answer;
    {
        let scoped_struct = int_struct;
        answer = some_process(&a, &scoped_struct);
    }
    println!("{}", answer);
}

