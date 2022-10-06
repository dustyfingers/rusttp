use std::net::TcpListener;
use std::io::Read;

// everything in a module is private by default

pub struct Server {
    address: String
}

// implementation block holds all of the functionality associated with the struct
// think of the struct itself holding the values and the impl block holding all of the associated functionality
impl Server {
    // methods vs functions

    // methods in rust are defined in the context of a struct and ALWAYS 
    // take a special self value (a reference to the instance of the struct that called it)
    // similar to the methods on objects in languages like java or c++

    // functions in rust do not require an instance of self

    // new is a function because we call it on the struct itself - not an instance of a struct
    // Self is an alias in every struct for its own type. eg -> Self == -> Server
    pub fn new(address: String) -> Self {
        Server {
            // address: address
            // we can omit the assignment if the name is the same, like js!
            address
        }
    }

    // run is a method because we call it on an instance of a Server
    // it takes self as the first param
    // we want the instance of the server to be deallocated when the run functions scope closes
    // so we dont need to pass a reference - we can pass the value directly in here and 
    // let the run function take ownership of the self variable 
    pub fn run(self) {
        // .bind method returns a result 
        // a result contains either an Ok() wrapping what the method returns or an Err()
        // unwrap extracts the value if the result contains an Ok()
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("Listening on port {}", self.address);

        loop {
            match listener.accept() {
                Ok((mut stream, clientAddress)) => {
                    // creates a buffer array of zeros of size 1024
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e)
            }
            // bad way to deal with results in rust
            // // hang and accept the next tcp request that comes in
            // let res = listener.accept();

            // // break from this iteration of loop if connection fails
            // if res.is_err() {
            //     continue;
            // }

            // 
        }
    }
}

