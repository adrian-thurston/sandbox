use flux;

fn main()
{
    println!("hello from main");

	flux::hello_from_flux();
	flux::hello_from_core();

	let bc = flux::alloc_struct_core();

	println!("main: {:?}", bc);

	flux::print_core(bc);
}
