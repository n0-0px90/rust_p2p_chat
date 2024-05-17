use std::io::{Read,Write};
use std::net::{TcpStream,TcpListener};

fn write_stream(mut stream: TcpStream){
    loop {
        // Declares new string to write to, reads user input, stores it as bytes in array
        let mut host_input = String::new();
        print!("Server> ");
        let _ = std::io::stdout().flush();
        std::io::stdin().read_line(&mut host_input).expect("Failed to read line.");
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
/** NOTES
* implement aes_gcm_siv streams, for decode
* kasm
* store output to then provide further enumeration
* banditctf //over the wire
* ncc
* code wars
* sector 7: assembly dev
* Implement a way to manage session keys
*/
//Entry point to prog
fn main() {
    //binding server at :443, implement client outreach to port.
    let listener = TcpListener::bind("127.0.0.1:443").expect("Failed to bind to 127.0.0.1:443");
    println!("Listening on 127.0.0.1:443" );
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
