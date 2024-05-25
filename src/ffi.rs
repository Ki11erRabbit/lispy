use crate::interpreter::{context::Context, kwargs::Kwargs, value::{CFunctionOutput, Function, FunctionShape, Value}};




pub fn get_ffi_library_test(context: &mut Context) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
	let lib = libloading::Library::new("ffi.so")?;
	let func = lib.get::<unsafe extern "C" fn(*mut Context, *mut Value, usize, *mut Kwargs, *mut CFunctionOutput)>(b"hello_c")?;
	let shape = FunctionShape::new(vec![]);
	let function = Function::CNative(*func, shape);
	context.define("hello-c", Value::new_function(function, context));
	Ok(())
    }
}
