#[macro_use]
extern crate prettytable;

mod ch2 {
    use std::time::{Duration, Instant};
    use num::Complex;
    use prettytable::{Cell, Row, Table};

    pub fn p221() {
        let a = 10;
        let b: i32 = 20;
        let c = 30i32;
        let d = 30_i32;
        let e = add(add(a, b), add(c, d));

        println!("( a + b ) + ( c + d ) = {}", e);
    }

    fn add(i: i32, j: i32) -> i32 {
        i + j
    }

    pub fn p231() {
        let twenty = 20;
        let twenty_one: i32 = 21;
        let twenty_two = 22i32;

        let addition = twenty + twenty_one + twenty_two;
        println!(
            "{} + {} + {} = {}",
            twenty, twenty_one, twenty_two, addition
        );

        let one_million: i64 = 1_000_000;
        println!("{}", one_million.pow(2));

        let forty_twos = [42.0, 42f32, 42.0_f32];

        println!("{:02}", forty_twos[0]);
    }

    pub fn p232() {
        let three = 0b11;
        let thirty = 0o36;
        let three_hundred = 0x12C;

        println!("base 10: {} {} {}", three, thirty, three_hundred);
        println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
        println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
        println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
    }

    pub fn p2331() {
        let a: i32 = 10;
        let b: u16 = 100;

        if a < (b as i32) {
            println!("Ten is less than one hundered.");
        }

        let b_ = b.try_into().unwrap();

        if a < b_ {
            println!("Ten is less than one hundred.");
        }
    }

    pub fn p2332() {
        let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
        let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

        println!("abc (f32)");
        println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
        println!("         0.3: {:x}", (abc.2).to_bits());
        println!();

        println!("xyz (f64)");
        println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
        println!("         0.3: {:x}", (xyz.2).to_bits());
        println!();

        assert!(abc.0 + abc.1 == abc.2);
        // assert!(xyz.0 + xyz.1 == xyz.2); // this test will fail
    }

    pub fn p2333() {
        let result: f32 = 0.1 + 0.1;
        let desired: f32 = 0.2;
        let absolute_difference = (desired - result).abs();
        assert!(absolute_difference <= f32::EPSILON);

        let x1: f32 = (-42.0_f32).sqrt();
        // assert_eq!(x1, x1); // NAN values are never equal
        assert!(x1.is_nan());

        let x2: f32 = 1.0 / 0.0;
        // assert!(x2.is_finite()) ; // this test will fail
        assert!(x2.is_infinite());
    }

    pub fn p234() {
        let a = Complex { re: 2.1, im: -1.2 };
        let b = Complex::new(11.1, 22.2);
        let result = a + b;

        println!("{} + {}i", result.re, result.im); // 13.2 + 21i
    }

    pub fn p241() {
        let mut table = Table::new();

        table.add_row(row!["Shorthand", "Equivalent to", "Access"]);
        table.add_row(Row::new(vec![
            Cell::new("for item in collection"),
            Cell::new("for item in IntoIteration::into_iter(collection)"),
            Cell::new("Ownership"),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("for item in &collection"),
            Cell::new("for item in collection.iter()"),
            Cell::new("Read-only"),
        ]));
        // A more complicated way to add a row:
        table.add_row(Row::new(vec![
            Cell::new("for item in &mut collection"),
            Cell::new("for item in collection.iter_mut()"),
            Cell::new("Read-write"),
        ]));

        // Print the table to stdout
        table.printstd();

        /* ANONYMOUS LOOPS
        for _ in 0..10 {
            ...
        }
         */

        /* AVOID MANAGING AN INDEX VARIABLE
        let collection = [1, 2, 3, 4, 5];
        for i in 0..collection.len() {
            let item = collection[i];
            ...
        }
         */

        /* 2.4.2 Continue: Skipping the rest of the current iteration
            for n in 0..10 {
                if n % 2 == 0 {
                    continue;
                }
                ...
            }
         */
    }

    pub fn p243() {
        /*
        let mut samples = vec![];

        while samples.len() < 10 {
            let sample = take_sample();
            if is_oyutlier(sample) {
                continue;
            }

            samples.push(sample);
        }
         */

        //
        let mut count = 0;
        let time_limit = Duration::new(1, 0);
        let start = Instant::now();

        while (Instant::now() - start) < time_limit {
            count += 1;
        } 
        println!("{}", count);

        /* AVOID WHILE WHEN ENDLESSLY LOOPING
        while true {
            println!("Are we there yet?");
        }
        */
    }

    pub fn p244() {
        /*
        loop {
            let requester, request = accept_request();
            let result = process_request(request);
            send_response(requester, result);
        }
         */
    }

    pub fn p245() {
        /* 
        for (x, y) in (0..).zip(0..) {
            if x + y > 100 {
                break;
            }
            ...
        }
        */

        /* BREAK FROM NESTED LOOPS
        'outer: for x in 0.. {
            for y in 0.. {
                for z in 0.. {
                    if x + y + z > 1000 {
                        break 'outer;
                    }
                    ...
                }
            }
        }
        */
    }
}

fn main() {
    println!("____2.2.1 Defining variables and calling functions");
    ch2::p221();

    println!("\n____2.3.1 Integers and decimal (floating-point) numbers");
    ch2::p231();

    println!("\n____2.3.2 Integers with base 2, base 8, and base 16 notation");
    ch2::p232();

    println!("\n____2.3.3 Comparing numbers");
    ch2::p2331();

    println!("\n____2.3.3 Floating-point hazards");
    ch2::p2332();

    println!("\n____2.3.3 The rest...");
    ch2::p2333();

    println!("\n____2.3.4 Rational, complex numbers, and other numeric types");
    ch2::p234();

    println!("\n____2.4.1 For: The central pillar of iteration");
    ch2::p241();

    println!("\n____2.4.3 While: Looping until a condition chnages its state");
    println!(">>>> USING WHILE TO STOP ITERATING ONCE A DURATION IS REACHED");
    ch2::p243();

    println!("\n____2.4.4 Loop: The basis for Rust's looping constructs");
    ch2::p244();

    println!("\n____2.4.5 Break: Aborting a loop");
    ch2::p245();
}
