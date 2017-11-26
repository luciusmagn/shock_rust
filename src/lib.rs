extern crate libloading as lib;

fn call_dynamic() -> lib::Result<()> {
    let lib = lib::Library::new("ServerPlugins/RustShock.dll")?;
    unsafe {
        let func: lib::Symbol<unsafe extern fn(*const u8, extern fn() -> ())> = lib.get(b"Rusting")?;
        func("cmd1".as_ptr(), cmd1);
        func("cmd2".as_ptr(), cmd2);
        Ok(())
    }
}

#[no_mangle]
pub unsafe extern fn rusting() {
	if let Err(e) = call_dynamic() {
		println!("{}", e);
	}
}

#[no_mangle]
pub extern fn get_cmd_register(register_cmd: extern fn(*const u8, extern fn() -> ()) -> ()) {
	register_cmd("cmd1".as_ptr(), cmd1);
	register_cmd("cmd2".as_ptr(), cmd2);
}

#[no_mangle]
pub extern fn cmd1() {
	println!("Hello, I am command numero uno");
}

#[no_mangle]
pub extern fn cmd2() {
	println!("Hello, I am command zwei");
}