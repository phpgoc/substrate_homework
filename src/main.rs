use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    // 使用50字节缓冲区
    let mut data = [0 as u8; 50];
    while match stream.read(&mut data) {
        Ok(size) => {
            // 回声一切！
            stream.write(&data[0..size]).unwrap();
            println!("size = {}", size);
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            // 遇到错误关闭连接
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() -> std::io::Result<()> {
    //开启 TCP 监听
    let listener = TcpListener::bind("0.0.0.0:9527")?;
    println!("Server listening on port 9527");
    // 接受连接并处理它们，为每个连接生成一个新线程
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // 连接成功
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                // 连接失败
            }
        }
    }
    Ok(())
}
