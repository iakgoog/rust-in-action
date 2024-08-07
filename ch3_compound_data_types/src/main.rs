#![allow(unused_variables)] // Relaxes compiler warnings while working through ideas
#![allow(dead_code)]

mod ch3 {
    use std::fmt::Debug;

    #[derive(Debug)]
    struct Filez {
        name: String,
        data: Vec<u8>,
    }

    impl Filez {
        fn new(name: &str) -> Filez {
            Filez {
                name: String::from(name),
                data: Vec::new(),
            }
        }

        fn new_with_data(name: &str, data: &Vec<u8>) -> Filez {
            let mut f= Filez::new(name);
            f.data = data.clone();
            f
        }

        fn read(self: &Filez, save_to: &mut Vec<u8>) -> usize {
            let mut tmp = self.data.clone();
            let read_length = tmp.len();
            save_to.reserve(read_length);
            save_to.append(&mut tmp);
            read_length
        }
    }

    // type File = String;

    fn open(f: &mut Filez) -> bool {
        true
    }

    fn close(f: &mut Filez) -> bool {
        true
    }

    fn read(f: &Filez, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = f.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }
    
    // pub fn p31() {
    //     let mut f1 = File::from("f1.txt");
    //     open(&mut f1);
    //     // read(f1, vec![]);
    //     close(&mut f1);
    // }

    fn report<T: Debug>(item: T) {
        println!("{:?}", item);
    }

    fn clear(text: &mut String) -> () {
        *text = String::from("");
    }

    fn dead_end() -> ! {
        panic!("you have reached a dead end");
    }

    /* The following example creates an infinite loop that prevents the function from returning
    fn forever() -> ! {
        loop {
            ...
        };
    }
    */

    pub fn p32() {
        let f1 = Filez {
            name: String::from("f1.txt"),
            data: Vec::new(),
        };

        let f1_name = &f1.name;
        let f1_length = &f1.data.len();

        println!("{:?}", f1);
        println!("{} is {} bytes long", f1_name, f1_length);
    }

    struct Hostname(String);

    fn connect(host: Hostname) {             // <2>
        println!("connected to {}", host.0); // <3>
    }

    pub fn p322() {
        let ordinary_string = String::from("localhost");
        let host = Hostname(ordinary_string.clone());
        // connect(ordinary_string);
    }

    pub fn p323() {
        let mut f2 = Filez {
            name: String::from("2.txt"),
            data: vec![114, 117, 115, 116, 33],
        };

        let mut buffer: Vec<u8> = vec![];

        open(&mut f2);
        let f2_length = read(&f2, &mut buffer);
        close(&mut f2);

        let text = String::from_utf8_lossy(&buffer);

        println!("{:?}", f2);
        println!("{} is {} bytes long", &f2.name, f2_length);
        println!("{}", text);
    }

    pub fn p331() {
        let f3_data: Vec<u8> = vec![
            114, 117, 115, 116, 33, 64
        ];
        let mut f3 = Filez::new_with_data("3.txt", &f3_data);

        let mut buffer: Vec<u8> = vec![];

        open(&mut f3);
        let f3_length = f3.read(&mut buffer);
        close(&mut f3);

        let text = String::from_utf8_lossy(&buffer);

        println!("{:?}", f3);
        println!("{} is {} bytes long", &f3.name, f3_length);
        println!("{}", text);
    } 
}

fn main() {
    // println!("____3.1 Using plain functions to experiment with an API");
    // ch3::p31();

    // println!("\n____3.2 Modeling files with struct");
    // ch3::p32();

    println!("\n____3.2 Modeling files with struct");
    ch3::p323();

    println!("\n____3.3.1 Simplying object creation by implenenting new()");
    ch3::p331();
}
