// ########## CHAPTER 3 ###############
// ###### COMPOUND DATA TYPES ########
/*
- Composing data with structs
- Creating enumerated data types
- Adding methods and handling errors in a type-safe manner
- Defining and implementing common behavior with traits
- Understanding how to keep implementation details private
- Using cargo to build documentation for your project 

Two key building blocks: struct and enum
*/
/*
#![allow(unused_variables)] // relax compiler warnings

    type File = String;     // type alias

    fn open(f: &mut File) -> bool {
        true
    }

    fn close(f: &mut File) -> bool {
        true
    }

#[allow(dead_code)]         // relax compiler warning about unused functions
fn read(f: &mut File,
        save_to: &mut Vec<u8>) -> ! { // ! type indicates the function never returns
    unimplemented!()        // macro that crashes if encountered 
}

fn main() {
    let mut f1 = File::from("f1.txt"); // File inherits String's methods through type alias
    open(&mut f1);
    //read(f1, vec![]);
    close(&mut f1);
}
*/
// UNIT TYPE () - formally a zero-length tuple
// - expresses a function that returns no value
// - eg functions without a return type return (), as do expressions with a ;

// "NEVER" TYPE ! - indicates a function never returns

//----- 3.2 Modeling files with struct ------

// struct allows creation of a composite type 
// - analagous to object or record in other languages
/*
#![allow(unused_variables)]

    #[derive(Debug)]        // allows println! to print File
    struct File {
        name: String,   // fields require explicit lifetimes if they're a ref to another object
        data: Vec<u8>,  // here the field lifetimes are implicit
    }

    fn open(f: &mut File) -> bool {
        true
    }

    fn close(f: &mut File) -> bool {
        true
    }

    fn read(
        f: &File,
        save_to: &mut Vec<u8>,
    ) -> usize {
        let mut tmp = f.data.clone();
        let read_length = tmp.len();
    
        save_to.reserve(read_length); // reserve capacity for read_length amount of elements
        save_to.append(&mut tmp);
        read_length
    }

    fn main() {
        let mut f2 = File {
            name: String::from("2.txt"),
            data: vec![114, 117, 115, 116, 33],
        };

        let mut buffer: Vec<u8> = vec![];

        open(&mut f2);
        let f2_length = read(&f2, &mut buffer);
        close(&mut f2);

        let text = String::from_utf8_lossy(&buffer); // convert Vec to String, non-UTF-8 bytes replaced with ?

        println!("{:?}", f2);
        println!("{} is {} bytes long", &f2.name, f2_length);
        println!("{}", text)
    }
*/
    //     let f1 = File {
    //         name: String::from("f1.txt"),  // generate owned strings from string literals
    //         data: Vec::new(),              // simulate empty file
    //     };

    //     let f1_name = &f1.name;            // accessing fields by reference prevents use after move
    //     let f1_length = &f1.data.len();

    //     println!("{:?}", f1);
    //     println!("{} is {} bytes long", f1_name, f1_length);
    // }
    
// NEWTYPE PATTERN
// wrap a core type with a struct to create a new type
/*
  struct Hostname(String); // hostname will be our new type

    fn connect(host: Hostname) {             // creates host of type Hostname
        println!("connected to {}", host.0); // access host's data
    }

    fn main() {
        let ordinary_string = String::from("localhost"); // get the string
        let host = Hostname ( ordinary_string.clone() ); // create a Hostname type using a copy of the string

        connect(ordinary_string); // this will generate an error as it is a String and not a Hostname type
}
*/

// ------ 3.3 Adding methods to a struct with impl --------
/* 
- Methods are functions coupled to some object
- Syntactically speaking, they are functions that don't need to specify one of their arguments
    - e.g. read(f, buffer) can be simplified as f.read(buffer)
- Rust does not contain the class keyword - unlike other languages that support methods
    - struct and enum can seem like classes but do not support injeritance
- methods are defined with impl blocks, which are distinct from struc/enum

- Rust:
    struct/enum File{
        Data
    }

    impl File {
        Methods
    }

- Classes in other languages:
    class File {
        Data
        Methods
    }

- the new() method is typically used to create objects

- literal syntax:
    File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

- File::new() syntax:
    File::new("f1.txt", vec![]);
*/

#[derive(Debug)]
    struct File {
        name: String,
        data: Vec<u8>,
    }

    impl File {
        fn new(name: &str) -> File {
            File {
                name: String::from(name),
                data: Vec::new(),
            }
        }
    }

    fn main() {
        let f3 = File::new("f3.txt");

        let f3_name = &f3.name;
        let f3_length = f3.data.len();

        println!("{:?}", f3);
        println!("{} is {} bytes long", f3_name, f3_length);
    }
    connect(ordinary_string);
}
*/

// ------ 3.4 RETURNING ERRORS ------
/*
Dealing with hardware is unreliable. There may be hardware faults, OS permission issues, etc.

A simple method to signal an error is to check the value of a global variable.

*/
// A Rust version of global variable error check:
/*
static mut ERROR: i32 = 0; // global variable with a static lifetime valid for life of the program

// ...

fn main() {
    let mut f = File::new("something.txt");

    read(f, buffer);
    unsafe {            // modifying static mut variables requires unsafe
        if ERROR != 0 {
            panic!("An error has occurred while reading the file")
        }
    }

    close(f);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred while closing the file")
        }
    }
}
*/
/*
use rand::{random};

    static mut ERROR: isize = 0;

    struct File;

    #[allow(unused_variables)]
    fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
        if random() && random() && random() { // random() is a coin flip, this will be true 1/8 times
            unsafe {
                ERROR = 1;
            }
        }
        0
    }

#[allow(unused_mut)]
fn main() {
    let mut f = File;
    let mut buffer = vec![];

    read(&f, &mut buffer);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred!")
        }
    }
}

*/

/* ----- CONST VS LET -----
- let allows interior mutability
- at the compiler level, let allows aliasing - referencing - the same data 
in multiple locations simultaneously
- mutable borrows never alias data
*/

/* ------ RESULT ------
Error handling uses a type that stands for both the standard and error case - the Result type
Result has two states - Ok and Err

Functions that interact with the file system return Result<File, String>
- When the function succeeds, it returns Ok(File)
- When it fails, it returns Err(String) - string allows an easy way to report error messages
- Unwrap() is needed to extract the value. It unwraps Ok(File) to produce File and crashes with Err(String)


*/
/*
use rand::prelude::*;

    fn one_in(denominator: u32) -> bool {
        thread_rng().gen_ratio(1, denominator)
    }

    #[derive(Debug)]
    struct File {
        name: String,
        data: Vec<u8>,
    }

    impl File {
        fn new(name: &str) -> File {
            File {
                name: String::from(name),
                data: Vec::new()
            }
        }
    

        fn new_with_data(name: &str, data: &Vec<u8>) -> File {
            let mut f = File::new(name);
            f.data = data.clone();
            f
        }

        fn read (
            self: &File,
            save_to: &mut Vec<u8>,
        ) -> Result<usize, String> { // Ok is usize, Err is String
            let mut tmp = self.data.clone();
            let read_length = tmp.len();
            save_to.reserve(read_length);
            save_to.append(&mut tmp);
            Ok(read_length)
        }
    }

    fn open(f: File) -> Result<File, String> {
        if one_in(10_000) { // Generates error with probability 1/10000
            let err_msg = String::from("Permission denied");
            return Err(err_msg);
        }
        Ok(f)
    }

    fn open(f: File) -> Result<File, String> {
        if one_in(100_000) {
            let err_msg = String::from("Interrupted by signal!");
            return Err(err_msg);
        }
        Ok(f)
    }

    fn main() {
        let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
        let mut f4 = File::new_with_data("4.txt", &f4_data);

        let mut buffer: Vec<u8> = vec![]; // placeholder

        f4 = open(f4).unwrap();
        let f4_length = f4.read(&mut buffer).unwrap();
        f4 = close(f4).unwrap();

        let text = String::from_utf8_lossy(&buffer);

        println!("{:?}", f4);
        println!("{} is {} bytes long", &f4.name, f4_length);
        println!("{}", text);
    }

*/// Result is an enum defined in the standard library.

// -------- ENUM ---------
/*
An enum is a type that can represent multiple known variants
eg:
enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}
*/

// Using enum to parse an event log
#[derive(Debug)]
enum Event {
    Update,
    Delete,
    Unknown,
}

type Message = String;

fn parse_log(line: &str) -> (Event, Message) {
    let parts: Vec<_> = line // infer element type
                        .splitn(2, ' ')
                        .collect(); // takes an iterator and returns Vec
    if parts.len() == 1 { // error if the line isn't split
        return (Event::Unknown, String::from(line))
    } 

    let event = parts(0);
    let rest = String::from(parts[1]);

    match event {
        "UPDATE" | "update" => (Event::Update, rest),
        "DELETE" | "delete" => (Event::Delete, rest),
        _ => (Event::Unknown, String::from(line)),
    }
}

fn main() {
    let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";

    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("{:?}", parse_result);
    }
}

// enums work with Rust's pattern-matching capabilities
// enums, like structs, support methods via impl blocks
// Rust's enums are more powerful than a set of constants

// enum variants can contain data:
enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

enum Card {
    King(Suit),
    Queen(Suit),
    Jack(Suit),
    Ace(Suit),
    Pip(Suit,usize),
}

// Using an enum to manage internal state