use std::io::{Read,Write,stdin, stdout};
use std::net::{TcpStream,TcpListener};

fn write_stream(mut stream: TcpStream){
    loop {
        // Declares new string to write to, reads user input, stores it as bytes in array
        let mut host_input = String::new();
        print!("Server> ");
        let _ = stdout().flush();
        stdin().read_line(&mut host_input).expect("Failed to read line.");
        let message = host_input.as_bytes();
        stream.write(message).expect("failed to write message.");
    }
}

fn read_stream (mut stream: TcpStream){
    loop {
        let mut _net_buffer = [0u8; 1024];
        stream.read(&mut _net_buffer).expect("Failed to read line.");
        let incoming_message = String::from_utf8_lossy(&_net_buffer[..]);
        println!("\n");
        print!("Client> {}", incoming_message);
    }
}

fn handle_client(stream: TcpStream){
    let write_clone: TcpStream = stream.try_clone().expect("Failed to clone to write");
    let read_clone: TcpStream = stream.try_clone().expect("Failed to clone to read");
    std::thread::spawn(|| read_stream(read_clone));
    std::thread::spawn(|| write_stream(write_clone));
}

//Entry point to prog
fn main() {
    //binding server at :443, implement client outreach to port.
    let listener = TcpListener::bind("0.0.0.0:443").expect("Failed to bind");
    let listening = listener.local_addr().unwrap();
    println!("Listening on {}", listening );
    //Iterates through incoming streams from client
    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e)
            }
        }
    }
}
