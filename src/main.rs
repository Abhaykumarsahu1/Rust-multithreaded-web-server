use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpStream, TcpListener}
};
fn main() {
    //below what's happening is that i want my program to listen for tcp connection comming from port no. 7878 on this machine
    //bind returns Result<TcpListener, io::Error> so i need to unwrap it, if another program is already listening on 7878, binding fails.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); 
    println!("Listening on http://127.0.0.1:7878");

    //here incoming gives an iterator of upcoming connection attempts, each item is essentially trying to connect to the server and each time returns a Result<TcpStream, io::Error>
    //so i need to unwrap each item
    for stream in listener.incoming(){

        let stream = stream.unwrap(); //RETURNS TcpStream which i can use to send data to the client

        println!("connection established");
        handle_connection(stream);
    }

}


fn handle_connection(mut stream: TcpStream){
    //so the rust program is basically acting as a server, it listens for incoming connections and when it gets one, it handles it
    //so i need to read the data from the client and send it back to him in a response 
    let buf_reader = BufReader::new(&stream);
    // let http_request: Vec<_> = buf_reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();

    let request_line = buf_reader.lines().next().unwrap().unwrap(); //here next returns an Option<Result<String, Error>>, was there even a line to begin with next queries then after succedd it goes to Result<> did reading that line succed that's why two unwraps

    // println!("Request: {http_request:#?}"); //here we basically have the GET request from the client

    let (status_line, filename) = if request_line == "GET / HTTP/1.1"{
        ("HTTP/1.1 200 OK", "hello.html")
    } else{
        ("HTTP/1.1 404 Not Found", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    
}