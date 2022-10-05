fn main() {
    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;
    // Server is a struct here - kind of like a class in oo languages
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
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
    fn new(address: String) -> Self {
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
    fn run(self) {
        println!("Listening on port {}", self.address);
    }
}


// sample HTTP request
/*
    GET /user?id=10 HTTP/1.1\r\n
    HEADERS \r\n
    BODY
*/

enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

struct Request {
    path: String,
    // query string can be 'null' - except rust has no null value
    // instead we have the Option type which can either contain a 'None' value
    // or a Some() value which wraps a value of the specified type, which here is a String
    query_string: Option<String>,
    // method can only be a certain set of strings
    method: Method
}
