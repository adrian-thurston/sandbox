#[no_mangle]
pub unsafe extern "C" fn hello_from_rust() {
    println!( "hello from rust" );
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

