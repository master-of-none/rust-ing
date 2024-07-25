use std::net::TcpListener;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connection has been successfully established!")
    }
}
