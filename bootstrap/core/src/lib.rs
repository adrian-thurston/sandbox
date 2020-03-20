use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Core
{
	i: i64,
	j: i64,
	map: BTreeMap<i64, i64>,
}

static STATIC_THING: u64 = 1;

#[no_mangle]
pub extern "C" fn hello_from_core()
{
	println!( "hello from core, static object at {:p}", &STATIC_THING as *const _ as *const _ );
}

pub fn print_core( core: Box<Core> )
{
	println!( "core: {:?}", core );
}

#[no_mangle]
pub extern "C" fn alloc_struct_core() -> Box<Core>
{
    println!( "allocating a struct from rust and returning it" );

	Box::new(Core {
		i: 1,
		j: 2,
		map: BTreeMap::<i64, i64>::new(),
	})
}

#[no_mangle]
pub extern "C" fn free_struct_core(_: Option<Box<Core>>)
{
    println!( "freeing struct from rust" );
}
