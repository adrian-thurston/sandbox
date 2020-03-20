pub use core::*;

#[no_mangle]
pub extern "C" fn hello_from_flux() {
    hello_from_core();
    println!("hello from flux");

    let buf = include_bytes!(concat!(env!("OUT_DIR"), "/string.data"));
    println!("got the bytes: {:?}", String::from_utf8(buf.to_vec()));
}
