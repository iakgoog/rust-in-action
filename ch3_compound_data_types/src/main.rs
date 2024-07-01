#![allow(unused_variables)] // Relaxes compiler warnings while working through ideas
#![allow(dead_code)]

mod ch3 {
    use std::fmt::Debug;

    use crate::Filez;

    type File = String;

    fn open(f: &mut File) -> bool {
        true
    }

    fn close(f: &mut File) -> bool {
        true
    }

    #[allow(dead_code)]
    fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
        unimplemented!()
    }
    
    pub fn p31() {
        let mut f1 = File::from("f1.txt");
        open(&mut f1);
        // read(f1, vec![]);
        close(&mut f1);
    }

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
}

fn main() {
    println!("Using plain functions to experiment with an API");
    ch3::p31();
}
