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
use crate::interpreter::InterpreterResult;


fn stdlib_string_to_ipv4_shape() -> FunctionShape {
    FunctionShape::new(vec!["string".to_string()])
}

fn stdlib_string_to_ipv4(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
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

fn stdlib_string_to_ipv6(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
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

fn stdlib_string_to_socket_addrv4(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    let string = if let Some(string) = args.get(0) {
	string.get_string(context)?.clone()
    } else if let Some(string) = keyword_args.get("string") {
	string.get_string(context)?.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","string->socket-addr-v4"], "string is not provided", context)));
    };

    let addr = string.parse::<std::net::SocketAddr>().map_err(|_| Exception::new(&vec!["network","string->socket-addr-v4"], "invalid socket address", context))?;
    let addr = Box::new(addr);

    let addr = Value::new_rust_value(addr, context);
    Ok(addr)
}

fn stdlib_string_to_socket_addrv6(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
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

fn stdlib_udp_socket(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
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
    let socket = Box::new(socket);

    let socket = Value::new_rust_value(socket, context);
    Ok(socket)
}

fn stdlib_tcp_socket_shape() -> FunctionShape {
    FunctionShape::new(vec!["addr".to_string()])
}

fn stdlib_tcp_socket(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    let addr = if let Some(addr) = args.get(0) {
	addr.clone()
    } else if let Some(addr) = keyword_args.get("addr") {
	addr.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","tcp-socket"], "addr is not provided", context)));
    };

    let addr = addr.get_rust_value(context)?;
    let socket = if let Some(addr) = addr.downcast_ref::<std::net::SocketAddrV4>() {
	std::net::TcpStream::connect(addr).map_err(|_| Exception::new(&vec!["network","tcp-socket"], "socket connect error", context))?
    } else if let Some(addr) = addr.downcast_ref::<std::net::SocketAddrV6>() {
	std::net::TcpStream::connect(addr).map_err(|_| Exception::new(&vec!["network","tcp-socket"], "socket connect error", context))?
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

fn stdlib_tcp_listener(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    let addr = if let Some(addr) = args.get(0) {
	addr.clone()
    } else if let Some(addr) = keyword_args.get("addr") {
	addr.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","tcp-listener"], "addr is not provided", context)));
    };

    let addr = addr.get_rust_value(context)?;
    let listener = if let Some(addr) = addr.downcast_ref::<std::net::SocketAddrV4>() {
	std::net::TcpListener::bind(addr).map_err(|_| Exception::new(&vec!["network","tcp-listener"], "listener bind error", context))?
    } else if let Some(addr) = addr.downcast_ref::<std::net::SocketAddrV6>() {
	std::net::TcpListener::bind(addr).map_err(|_| Exception::new(&vec!["network","tcp-listener"], "listener bind error", context))?
    } else {
	return Err(Box::new(Exception::new(&vec!["network","tcp-listener"], "addr is not a socket address", context)));
    };
    let listener = Box::new(listener);

    let listener = Value::new_rust_value(listener, context);
    Ok(listener)
}

fn stdlib_accept_shape() -> FunctionShape {
    FunctionShape::new(vec!["listener".to_string()])
}

fn stdlib_accept(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    let listener = if let Some(listener) = args.get(0) {
	listener.clone()
    } else if let Some(listener) = keyword_args.get("listener") {
	listener.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","accept"], "listener is not provided", context)));
    };

    let listener = listener.get_rust_value(context)?;
    let listener = listener.downcast_ref::<std::net::TcpListener>().ok_or(Box::new(Exception::new(&vec!["network","accept"], "listener is not a tcp listener", context)))?;
    let (stream, addr) = listener.accept().map_err(|err| match err {
	e if e.kind() == std::io::ErrorKind::WouldBlock => Exception::new(&vec!["network","accept"], "accept would block", context),
	_ => Exception::new(&vec!["network","accept"], "accept error", context)
	})?;
    let stream = Box::new(stream);
    let addr = Box::new(addr);

    let stream = Value::new_rust_value(stream, context);
    let addr = Value::new_rust_value(addr, context);
    Ok(Value::new_pair(stream, addr, context))
}

fn stdlib_set_nonblocking_shape() -> FunctionShape {
    FunctionShape::new(vec!["socket".to_string(), "nonblocking".to_string()])
}

fn stdlib_set_nonblocking(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
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
    if let Some(socket) = socket.downcast_ref::<std::net::UdpSocket>() {
	socket.set_nonblocking(nonblocking).map_err(|_| Exception::new(&vec!["network","set-nonblocking"], "set nonblocking error", context))?;
    } else if let Some(socket) = socket.downcast_ref::<std::net::TcpStream>() {
	socket.set_nonblocking(nonblocking).map_err(|_| Exception::new(&vec!["network","set-nonblocking"], "set nonblocking error", context))?;
    } else {
	return Err(Box::new(Exception::new(&vec!["network","set-nonblocking"], "socket is not a socket", context)));
    };

    Ok(Value::new_nil())
}

fn stdlib_send_shape() -> FunctionShape {
    FunctionShape::new(vec!["socket".to_string(), "data".to_string()])
}

fn stdlib_send(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    let socket = if let Some(socket) = args.get(0) {
	socket.clone()
    } else if let Some(socket) = keyword_args.get("socket") {
	socket.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","send-to"], "socket is not provided", context)));
    };

    let data = if let Some(data) = args.get(1) {
	data.get_string(context)?.clone()
    } else if let Some(data) = keyword_args.get("data") {
	data.get_string(context)?.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","send-to"], "data is not provided", context)));
    };

    let socket = socket.get_rust_value(context)?;
    if let Some(socket) = socket.downcast_ref::<std::net::UdpSocket>() {
	socket.send(&data.as_bytes()).map_err(|_| Exception::new(&vec!["network","send-to"], "send error", context))?;
    } else if let Some(mut socket) = socket.downcast_ref::<std::net::TcpStream>() {
	socket.write_all(&data.as_bytes()).map_err(|_| Exception::new(&vec!["network","send-to"], "send error", context))?;
    } else {
	return Err(Box::new(Exception::new(&vec!["network","send-to"], "socket is not a socket", context)));
    };
    Ok(Value::new_nil())
}

fn stdlib_receive_shape() -> FunctionShape {
    FunctionShape::new(vec!["socket".to_string()])
}

fn stdlib_receive(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    let socket = if let Some(socket) = args.get(0) {
	socket.clone()
    } else if let Some(socket) = keyword_args.get("socket") {
	socket.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","receive-from"], "socket is not provided", context)));
    };
    todo!("add bytevector");
    let socket = socket.get_rust_value(context)?;
    let mut buffer = [0; 1024];
    let data = if let Some(socket) = socket.downcast_ref::<std::net::UdpSocket>() {
	let (size, _) = socket.recv_from(&mut buffer).map_err(|_| Exception::new(&vec!["network","receive-from"], "receive error", context))?;
	std::str::from_utf8(&buffer[..size]).map_err(|_| Exception::new(&vec!["network","receive-from"], "invalid utf8", context))?.to_string()
    } else if let Some(mut socket) = socket.downcast_ref::<std::net::TcpStream>() {
	let size = socket.read(&mut buffer).map_err(|_| Exception::new(&vec!["network","receive-from"], "receive error", context))?;
	std::str::from_utf8(&buffer[..size]).map_err(|_| Exception::new(&vec!["network","receive-from"], "invalid utf8", context))?.to_string()
    } else {
	return Err(Box::new(Exception::new(&vec!["network","receive-from"], "socket is not a socket", context)));
    };
    let data = Value::new_string(&data, context);
    Ok(data)
}