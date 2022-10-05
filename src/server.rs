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
        println!("Listening on port {}", self.address);
    }
}
