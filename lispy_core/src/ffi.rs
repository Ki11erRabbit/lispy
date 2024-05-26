


extern "C" {
    #[no_mangle]
    pub fn load_module(module_name: *mut String, context: *mut Context);
}




pub fn load_dynamic_libs(context: &mut Context, so_load_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
	let lib = libloading::Library::new("ffi.so")?;
	let func = lib.get::<unsafe extern "C" fn(*mut Context, *mut Value, usize, *mut Kwargs, *mut CFunctionOutput)>(b"hello_c")?;
	let shape = FunctionShape::new(vec![]);
	let function = Function::CNative(*func, shape);
	let function = Value::new_function(function, context);
	context.define("hello-c", function);
    }
    Ok(())
}
