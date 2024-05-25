use std::io::Write;
use std::io::Read;
use std::collections::HashMap;
use crate::interpreter::Exception;
use crate::interpreter::HelperResult;
use crate::interpreter::value::Function;
use crate::interpreter::value::FunctionShape;
use crate::interpreter::value::Value;
use crate::interpreter::context::Context;
use crate::interpreter::module::Module;
use crate::interpreter::kwargs::Kwargs;


fn stdlib_string_to_ipv4_shape() -> FunctionShape {
    FunctionShape::new(vec!["string".to_string()])
}

fn stdlib_string_to_ipv4(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let string = if let Some(string) = args.get(0) {
	string.get_string(context)?.clone()
    } else if let Some(string) = keyword_args.get("string") {
	string.get_string(context)?.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","string->ipv4"], "string is not provided", context)));
    };

    let ip = string.parse::<std::net::Ipv4Addr>().map_err(|_| Exception::new(&vec!["network","string->ipv4"], "invalid ip address", context))?;
    let ip = Box::new(ip);
    
    let ip = Value::new_rust_value(ip, context);
    Ok(ip)
}

fn stdlib_string_to_ipv6_shape() -> FunctionShape {
    FunctionShape::new(vec!["string".to_string()])
}

fn stdlib_string_to_ipv6(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let string = if let Some(string) = args.get(0) {
	string.get_string(context)?.clone()
    } else if let Some(string) = keyword_args.get("string") {
	string.get_string(context)?.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","string->ipv6"], "string is not provided", context)));
    };

    let ip = string.parse::<std::net::Ipv6Addr>().map_err(|_| Exception::new(&vec!["network","string->ipv6"], "invalid ip address", context))?;
    let ip = Box::new(ip);

    let ip = Value::new_rust_value(ip, context);
    Ok(ip)
}

fn stdlib_string_to_socket_addr_shape() -> FunctionShape {
	FunctionShape::new(vec!["string".to_string()])
}

fn stdlib_string_to_socket_addrv4(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let string = if let Some(string) = args.get(0) {
	string.get_string(context)?.clone()
    } else if let Some(string) = keyword_args.get("string") {
	string.get_string(context)?.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","string->socket-addr-v4"], "string is not provided", context)));
    };
    
    let addr = string.parse::<std::net::SocketAddrV4>().map_err(|_| Exception::new(&vec!["network","string->socket-addr-v4"], "invalid socket address", context))?;
    let addr = Box::new(addr);

    let addr = Value::new_rust_value(addr, context);
    Ok(addr)
}

fn stdlib_string_to_socket_addrv6(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let string = if let Some(string) = args.get(0) {
	string.get_string(context)?.clone()
    } else if let Some(string) = keyword_args.get("string") {
	string.get_string(context)?.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","string->socket-addr-v6"], "string is not provided", context)));
    };

    let addr = string.parse::<std::net::SocketAddrV6>().map_err(|_| Exception::new(&vec!["network","string->socket-addr-v6"], "invalid socket address", context))?;
    let addr = Box::new(addr);

    let addr = Value::new_rust_value(addr, context);
    Ok(addr)
}

fn stdlib_udp_socket_shape() -> FunctionShape {
    FunctionShape::new(vec!["addr".to_string()])
}

fn stdlib_udp_socket(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let addr = if let Some(addr) = args.get(0) {
	addr.clone()
    } else if let Some(addr) = keyword_args.get("addr") {
	addr.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","udp-socket"], "addr is not provided", context)));
    };

    let addr = addr.get_rust_value(context)?;
    let socket = if let Some(addr) = addr.downcast_ref::<std::net::SocketAddrV4>() {
	std::net::UdpSocket::bind(addr).map_err(|_| Exception::new(&vec!["network","udp-socket"], "socket bind error", context))?
    } else if let Some(addr) = addr.downcast_ref::<std::net::SocketAddrV6>() {
	std::net::UdpSocket::bind(addr).map_err(|_| Exception::new(&vec!["network","udp-socket"], "socket bind error", context))?
    } else {
	return Err(Box::new(Exception::new(&vec!["network","udp-socket"], "addr is not a socket address", context)));
    };
    let socket = Box::new(Some(socket));

    let socket = Value::new_rust_value(socket, context);
    Ok(socket)
}

fn stdlib_tcp_socket_shape() -> FunctionShape {
    FunctionShape::new(vec!["addr".to_string()])
}

fn stdlib_tcp_socket(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let addr = if let Some(addr) = args.get(0) {
	addr.clone()
    } else if let Some(addr) = keyword_args.get("addr") {
	addr.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","tcp-socket"], "addr is not provided", context)));
    };

    let addr = addr.get_rust_value(context)?;
    let socket = if let Some(addr) = addr.downcast_ref::<std::net::SocketAddrV4>() {
	std::net::TcpStream::connect(addr).map_err(|err| Exception::new(&vec!["network","tcp-socket"], &format!("{}", err), context))?
    } else if let Some(addr) = addr.downcast_ref::<std::net::SocketAddrV6>() {
	std::net::TcpStream::connect(addr).map_err(|err| Exception::new(&vec!["network","tcp-socket"], &format!("{}", err), context))?
    } else {
	return Err(Box::new(Exception::new(&vec!["network","tcp-socket"], "addr is not a socket address", context)));
    };
    let socket = Box::new(socket);

    let socket = Value::new_rust_value(socket, context);
    Ok(socket)
}

fn stdlib_tcp_listener_shape() -> FunctionShape {
    FunctionShape::new(vec!["addr".to_string()])
}

fn stdlib_tcp_listener(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let addr = if let Some(addr) = args.get(0) {
	addr.clone()
    } else if let Some(addr) = keyword_args.get("addr") {
	addr.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","tcp-listener"], "addr is not provided", context)));
    };

    let addr = addr.get_rust_value(context)?;
    let listener = if let Some(addr) = addr.downcast_ref::<std::net::SocketAddrV4>() {
	std::net::TcpListener::bind(addr).map_err(|err| Exception::new(&vec!["network","tcp-listener"], &format!("{}", err), context))?
    } else if let Some(addr) = addr.downcast_ref::<std::net::SocketAddrV6>() {
	std::net::TcpListener::bind(addr).map_err(|err| Exception::new(&vec!["network","tcp-listener"], &format!("{}", err), context))?
    } else {
	return Err(Box::new(Exception::new(&vec!["network","tcp-listener"], "addr is not a socket address", context)));
    };
    let listener = Box::new(Some(listener));

    let listener = Value::new_rust_value(listener, context);
    Ok(listener)
}

fn stdlib_connect_shape() -> FunctionShape {
    FunctionShape::new(vec!["socket".to_string(), "addr".to_string()])
}

fn stdlib_connect(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let socket = if let Some(socket) = args.get(0) {
	socket.clone()
    } else if let Some(socket) = keyword_args.get("socket") {
	socket.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","connect"], "socket is not provided", context)));
    };

    let addr = if let Some(addr) = args.get(1) {
	addr.clone()
    } else if let Some(addr) = keyword_args.get("addr") {
	addr.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","connect"], "addr is not provided", context)));
    };

    let addr = addr.get_rust_value(context)?;

    if let Some(addr) = addr.downcast_ref::<std::net::SocketAddrV4>() {
	let socket = socket.get_rust_value(context)?;
	if let Some(socket) = socket.downcast_ref::<Option<std::net::UdpSocket>>() {
	    if let Some(socket) = socket {
		socket.connect(addr).map_err(|err| Exception::new(&vec!["network","connect"], &format!("{}", err), context))?;
	    } else {
		return Err(Box::new(Exception::new(&vec!["network","connect"], "socket is not a udp socket", context)));
	    }
	} else {
	    return Err(Box::new(Exception::new(&vec!["network","connect"], "socket is not a tcp stream", context)));
	};
    } else if let Some(addr) = addr.downcast_ref::<std::net::SocketAddrV6>() {
	let socket = socket.get_rust_value(context)?;
	if let Some(socket) = socket.downcast_ref::<Option<std::net::UdpSocket>>() {
	    if let Some(socket) = socket {
		socket.connect(addr).map_err(|err| Exception::new(&vec!["network","connect"], &format!("{}", err), context))?;
	    } else {
		return Err(Box::new(Exception::new(&vec!["network","connect"], "socket is not a udp socket", context)));
	    }
	} else {
	    return Err(Box::new(Exception::new(&vec!["network","connect"], "socket is not a tcp stream", context)));
	};
    } else {
	return Err(Box::new(Exception::new(&vec!["network","connect"], "addr is not a socket address", context)));
    }

    Ok(Value::new_nil())
}

fn stdlib_accept_shape() -> FunctionShape {
    FunctionShape::new(vec!["listener".to_string()])
}

fn stdlib_accept(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let listener = if let Some(listener) = args.get(0) {
	listener.clone()
    } else if let Some(listener) = keyword_args.get("listener") {
	listener.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","accept"], "listener is not provided", context)));
    };

    let listener = listener.get_rust_value(context)?;
    let listener = listener.downcast_ref::<Option<std::net::TcpListener>>().ok_or(Box::new(Exception::new(&vec!["network","accept"], "listener is not a tcp listener", context)))?;
    if listener.is_none() {
	return Err(Box::new(Exception::new(&vec!["network","accept"], "listener is closed", context)));
    }
    let listener = listener.as_ref().unwrap();
    let (stream, addr) = listener.accept().map_err(|err| Exception::new(&vec!["network","accept"], &format!("{}", err), context))?;
    let stream = Box::new(stream);
    let addr = Box::new(addr);

    let stream = Value::new_rust_value(stream, context);
    let addr = Value::new_rust_value(addr, context);
    Ok(Value::new_pair(stream, addr, context))
}

fn stdlib_set_nonblocking_shape() -> FunctionShape {
    FunctionShape::new(vec!["socket".to_string(), "nonblocking".to_string()])
}

fn stdlib_set_nonblocking(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let socket = if let Some(socket) = args.get(0) {
	socket.clone()
    } else if let Some(socket) = keyword_args.get("socket") {
	socket.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","set-nonblocking"], "socket is not provided", context)));
    };

    let nonblocking = if let Some(nonblocking) = args.get(1) {
	nonblocking.get_boolean(context)?
    } else if let Some(nonblocking) = keyword_args.get("nonblocking") {
	nonblocking.get_boolean(context)?
    } else {
	return Err(Box::new(Exception::new(&vec!["network","set-nonblocking"], "nonblocking is not provided", context)));
    };

    let socket = socket.get_rust_value(context)?;
    if let Some(socket) = socket.downcast_ref::<Option<std::net::UdpSocket>>() {
	if let Some(socket) = socket {
	    socket.set_nonblocking(nonblocking).map_err(|err| Exception::new(&vec!["network","set-nonblocking"], &format!("{}", err), context))?;
	} else {
	    return Err(Box::new(Exception::new(&vec!["network","set-nonblocking"], "socket is closed", context)));
	}
    } else if let Some(socket) = socket.downcast_ref::<std::net::TcpStream>() {
	socket.set_nonblocking(nonblocking).map_err(|err| Exception::new(&vec!["network","set-nonblocking"], &format!("{}", err), context))?;
    } else {
	return Err(Box::new(Exception::new(&vec!["network","set-nonblocking"], "socket is not a socket", context)));
    };

    Ok(Value::new_nil())
}

fn stdlib_send_shape() -> FunctionShape {
    FunctionShape::new(vec!["socket".to_string(), "data".to_string()])
}

fn stdlib_send(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let socket = if let Some(socket) = args.get(0) {
	socket.clone()
    } else if let Some(socket) = keyword_args.get("socket") {
	socket.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","send"], "socket is not provided", context)));
    };

    if let Some(data) = args.get(1) {
	if let Ok(data) = data.get_string(context) {
	    let socket = socket.get_rust_value(context)?;
	    if let Some(socket) = socket.downcast_ref::<Option<std::net::UdpSocket>>() {
		if let Some(socket) = socket {
		    socket.send(&data.as_bytes()).map_err(|err| Exception::new(&vec!["network","send"], &format!("{}", err), context))?;
		} else {
		    return Err(Box::new(Exception::new(&vec!["network","send"], "socket is closed", context)));
		}
	    } else if let Some(mut socket) = socket.downcast_ref::<std::net::TcpStream>() {
		socket.write_all(&data.as_bytes()).map_err(|err| Exception::new(&vec!["network","send"], &format!("{}", err), context))?;
	    } else {
		return Err(Box::new(Exception::new(&vec!["network","send"], "socket is not a socket", context)));
	    };
	} else if let Ok(data) = data.get_bytevector(context) {
	    let socket = socket.get_rust_value(context)?;
	    if let Some(socket) = socket.downcast_ref::<Option<std::net::UdpSocket>>() {
		if let Some(socket) = socket {
		    socket.send(&data).map_err(|err| Exception::new(&vec!["network","send"], &format!("{}", err), context))?;
		} else {
		    return Err(Box::new(Exception::new(&vec!["network","send"], "socket is closed", context)));
		}
	    } else if let Some(mut socket) = socket.downcast_ref::<std::net::TcpStream>() {
		socket.write_all(&data).map_err(|err| Exception::new(&vec!["network","send"], &format!("{}", err), context))?;
	    } else {
		return Err(Box::new(Exception::new(&vec!["network","send"], "socket is not a socket", context)));
	    };
	} else {
	    return Err(Box::new(Exception::new(&vec!["network","send"], "data is not a string or bytevector", context)));
	}
    } else if let Some(data) = keyword_args.get("data") {
	if let Ok(data) = data.get_string(context) {
	    let socket = socket.get_rust_value(context)?;
	    if let Some(socket) = socket.downcast_ref::<Option<std::net::UdpSocket>>() {
		if let Some(socket) = socket {
		    socket.send(&data.as_bytes()).map_err(|err| Exception::new(&vec!["network","send"], &format!("{}", err), context))?;
		} else {
		    return Err(Box::new(Exception::new(&vec!["network","send"], "socket is closed", context)));
		}
	    } else if let Some(mut socket) = socket.downcast_ref::<std::net::TcpStream>() {
		socket.write_all(&data.as_bytes()).map_err(|err| Exception::new(&vec!["network","send"], &format!("{}", err), context))?;
	    } else {
		return Err(Box::new(Exception::new(&vec!["network","send"], "socket is not a socket", context)));
	    };
	} else if let Ok(data) = data.get_bytevector(context) {
	    let socket = socket.get_rust_value(context)?;
	    if let Some(socket) = socket.downcast_ref::<Option<std::net::UdpSocket>>() {
		if let Some(socket) = socket {
		    socket.send(&data).map_err(|err| Exception::new(&vec!["network","send"], &format!("{}", err), context))?;
		} else {
		    return Err(Box::new(Exception::new(&vec!["network","send"], "socket is closed", context)));
		}
	    } else if let Some(mut socket) = socket.downcast_ref::<std::net::TcpStream>() {
		socket.write_all(&data).map_err(|err| Exception::new(&vec!["network","send"], &format!("{}", err), context))?;
	    } else {
		return Err(Box::new(Exception::new(&vec!["network","send"], "socket is not a socket", context)));
	    };
	} else {
	    return Err(Box::new(Exception::new(&vec!["network","send"], "data is not a string or bytevector", context)));
	}
    } else {
	return Err(Box::new(Exception::new(&vec!["network","send"], "data is not provided", context)));
    };
    Ok(Value::new_nil())
}

fn stdlib_receive_shape() -> FunctionShape {
    FunctionShape::new(vec!["socket".to_string()])
}

fn stdlib_receive(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let socket = if let Some(socket) = args.get(0) {
	socket.clone()
    } else if let Some(socket) = keyword_args.get("socket") {
	socket.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","receive"], "socket is not provided", context)));
    };

    let socket = socket.get_rust_value(context)?;
    let mut buffer = [0; 1024];
    let data = if let Some(socket) = socket.downcast_ref::<Option<std::net::UdpSocket>>() {
	if let Some(socket) = socket {
	    let (size, _) = socket.recv_from(&mut buffer).map_err(|err| Exception::new(&vec!["network","receive"], &format!("{}", err), context))?;
	    buffer[..size].to_vec()
	} else {
	    return Err(Box::new(Exception::new(&vec!["network","receive"], "socket is closed", context)));
	}
    } else if let Some(mut socket) = socket.downcast_ref::<std::net::TcpStream>() {
	let size = socket.read(&mut buffer).map_err(|err| Exception::new(&vec!["network","receive"], &format!("{}", err), context))?;
	buffer[..size].to_vec()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","receive"], "socket is not a socket", context)));
    };
    let data = Value::new_bytevector(data, context);
    Ok(data)
}

fn stdlib_close_shape() -> FunctionShape {
    FunctionShape::new(vec!["socket".to_string()])
}

fn stdlib_close(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let mut socket = if let Some(socket) = args.get(0) {
	socket.clone()
    } else if let Some(socket) = keyword_args.get("socket") {
	socket.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","close"], "socket is not provided", context)));
    };

    let socket = socket.get_rust_value_mut(context)?;
    if let Some(socket) = socket.downcast_mut::<Option<std::net::UdpSocket>>() {
	let socket = socket.take();
	if socket.is_none() {
	    return Err(Box::new(Exception::new(&vec!["network","close"], "socket is already closed", context)));
	}
    } else if let Some(socket) = socket.downcast_ref::<std::net::TcpStream>() {
	socket.shutdown(std::net::Shutdown::Both).map_err(|err| Exception::new(&vec!["network","close"], &format!("{}", err), context))?;
    } else if let Some(socket) = socket.downcast_mut::<Option<std::net::TcpListener>>() {
	let socket = socket.take();
	if socket.is_none() {
	    return Err(Box::new(Exception::new(&vec!["network","close"], "socket is already closed", context)));
	}
    } else {
	return Err(Box::new(Exception::new(&vec!["network","close"], "socket is not a socket", context)));
    };

    Ok(Value::new_nil())
}



pub fn get_network_library(context: &mut Context) -> Module {
    let submodules = HashMap::new();
    context.push_frame(None);

    context.define("string->ipv4", Value::new_function(Function::Native(stdlib_string_to_ipv4, stdlib_string_to_ipv4_shape()), context));
    context.define("string->ipv6", Value::new_function(Function::Native(stdlib_string_to_ipv6, stdlib_string_to_ipv6_shape()), context));
    context.define("string->socket-addr-v4", Value::new_function(Function::Native(stdlib_string_to_socket_addrv4, stdlib_string_to_socket_addr_shape()), context));
    context.define("string->socket-addr-v6", Value::new_function(Function::Native(stdlib_string_to_socket_addrv6, stdlib_string_to_socket_addr_shape()), context));
    context.define("udp-socket", Value::new_function(Function::Native(stdlib_udp_socket, stdlib_udp_socket_shape()), context));
    context.define("tcp-socket", Value::new_function(Function::Native(stdlib_tcp_socket, stdlib_tcp_socket_shape()), context));
    context.define("tcp-listener", Value::new_function(Function::Native(stdlib_tcp_listener, stdlib_tcp_listener_shape()), context));
    context.define("connect", Value::new_function(Function::Native(stdlib_connect, stdlib_connect_shape()), context));
    context.define("accept", Value::new_function(Function::Native(stdlib_accept, stdlib_accept_shape()), context));
    context.define("set-nonblocking", Value::new_function(Function::Native(stdlib_set_nonblocking, stdlib_set_nonblocking_shape()), context));
    context.define("send", Value::new_function(Function::Native(stdlib_send, stdlib_send_shape()), context));
    context.define("receive", Value::new_function(Function::Native(stdlib_receive, stdlib_receive_shape()), context));
    context.define("close", Value::new_function(Function::Native(stdlib_close, stdlib_close_shape()), context));

    let frame = context.pop_frame().expect("pop error");

    Module::new_loaded(submodules, frame)
}
