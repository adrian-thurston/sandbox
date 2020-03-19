use std::collections::BTreeMap;

#[allow(dead_code)]
pub struct Struct
{
	i: i64,
	j: i64,
	map: BTreeMap<i64, i64>,
}

#[no_mangle]
pub unsafe extern "C" fn hello_from_rust()
{
    println!( "hello from rust, my friends" );
}

static STATIC_THING: u64 = 1;

#[no_mangle]
pub unsafe extern "C" fn alloc_struct() -> Box<Struct>
{
	println!( "static object at {:p}", &STATIC_THING as *const _ as *const _ );
    println!( "allocating a struct from rust and returning it" );

	Box::new(Struct {
		i: 1,
		j: 2,
		map: BTreeMap::<i64, i64>::new(),
	})
}

#[no_mangle]
pub extern "C" fn free_struct(_: Option<Box<Struct>>)
{
    println!( "freeing struct from rust" );
	println!( "static object at {:p}", &STATIC_THING as *const _ as *const _ );
}

