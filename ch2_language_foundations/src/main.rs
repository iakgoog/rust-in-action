#[macro_use]
extern crate prettytable;

mod ch2 {
    use std::{ops::Add, time::{Duration, Instant}};
    use num::Complex;
    use prettytable::{Cell, Row, Table};
    use regex::Regex;

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

    pub fn p246() {
        /* 
        if item == 42 {
            ...
        }

        if item == 42 {
            ...
        } else if item == 132 {
            ...
        } else {
            ...
        }
        */

        // Rust has no concept of “truthy” or “falsey” types
        // Rust is an expression-based language

        // Rust programmers assign variables from conditional expressions
        let n = 123456;
        let description = if is_even(n) {
            "even"
        } else {
            "odd"
        };
        println!("{} is {}", n, description); // 123456 is even

        // This can be extended to other blocks including `match` like this
        let n2 = 654321;
        let description2 = match is_even(n2) {
            true => "even",
            false => "odd",
        };
        println!("{} is {}", n2, description2); // 654321 is odd

        // the `break` keyword also returns a value
        let n3 = loop {
            break 123;
        };
        println!("{}", n3);
    }

    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }

    pub fn p247() {
        /* 
        match item {
            0 => {},
            // To match a single value, provide the value. No operator is required.
            10 ..= 20 => {},
            // The ..=syntax matches an inclusive range.
            // Inclusive ranges (10 ..= 20) to match any value within the range.
            40 | 80 => {},
            // The vertical bar (|) matches values on either side of it.
            _ => {},
            // The underscore (_) matches every value.
        }
        */

        let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
        for item in &haystack {
            let result = match item {
                42 | 132 => "hit!",
                _ => "miss",
            };

            if result == "hit!" {
                println!("{}: {}", item, result);
            }
        }
    }

    pub fn p25() {
        // 	┼ ─ │ ├ ┤ ┌ ┐ └ ┘ ┴ ┬
        println!("  Identifier ┌Parameters┐ Return type");
        println!("        │  ┌─┴──┐  ┌────┴┐     │");
        println!("    fn add(i: i32, j: i32) -> i32 {{");
        println!("     │     │   │   │   │    │     │");
        println!("  Keyword  │  Type │ Type   │     │");
        println!("      Identifier Identifier │     │");
        println!("    Thin arrow to indicate return │");
        println!("                          Begin code block");
        // fn add(i: 32, j: i32) -> i32 {
    }

    pub fn p26() {
        let a = 42;
        let r = &a;          // r is a reference to a.
        let b = a + *r;       // Adds a to a (via dereferencing r) and assign it to b
        println!("a + a = {}", b); // Prints "a + a = 84"

        let needle = 0o204;
        let haystack= [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147] ;
        for item in &haystack { // Iterates over references to elements within haystack
            if *item == needle {      // The syntax *item returns the iterms' referent.
                println!("{}", item);
            }
        }
    }

    fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
        *i + *j
    }

    pub fn p281() {
        let a = 10;
        let b = 20;
        let res = add_with_lifetimes(&a, &b);
        println!("{}", res);
    }

    /*
    fn add<T>(i: T, j: T) -> T {
        i + j
    }
    error[E0369]: cannot add `T` to `T`
    This issue arises because T really means any type at all, even types where addition is not supported.
    */
    // The fragment <T: std::ops::Add<Output = T>> says that T must implement std::ops::Add.
    // Using a single type variable T with the trait bound ensures that arguments i and j,
    // as well as the result type, are the same type and that their type supports addition
    fn add2<T: Add<Output = T>>(i: T, j: T) -> T {
        i + j
    }
    // To reiterate: all of Rust’s operators are syntactic sugar for a trait’s methods.
    // Rust supports operator overloading this way. During the compilation process,
    // a + b is converted to a.add(b)

    pub fn p282() {
        let floats = add2(1.2, 3.4);
        let ints = add2(10, 20);
        let durations = add2(
            Duration::new(5, 0),
            Duration::new(10, 0)
        );

        println!("{}", floats);
        println!("{}", ints);
        println!("{:?}", durations);
    }

    pub fn p29() {
        let search_term = "picture";
        let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. 
What do we seek through millions of pages?";

        for line in quote.lines() {
            if line.contains(search_term) {
                println!("{}", line);
            }
        }

        let search_term2 = "picture";
        let quote2 = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek trhough millions of pages?";

        for (i, line) in quote2.lines().enumerate() {
            if line.contains(search_term2) {
                let line_num = i + 1;
                println!("{}: {}", line_num, line);
            }
        }
    }

    pub fn p2101() {
        let one = [1, 2, 3];
        let two: [u8; 3]  = [1, 2, 3];
        let blank1 = [0; 3];
        let blank2: [u8; 3]  = [0; 3];

        let arrays = [one, two, blank1, blank2];

        for a in &arrays {
            print!("{:?}:", a);
            for n in a.iter() {
                print!("\t{} + 10 = {}", n, n + 10);
            }
            let mut sum = 0;
            for i in 0..a.len() {
                sum += a[i];
            }
            println!("\t({:?} = {}", a, sum);
        }
    }

    pub fn p2103() {
        let ctx_lines = 2;
        let needle = "oo";
        let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

        let mut tags: Vec<usize> = vec![];
        let mut ctx: Vec<Vec<(usize, String)>> = vec![];
        for (i, line) in haystack.lines().enumerate() {
            if line.contains(needle) {
                tags.push(i);

                let v = Vec::with_capacity(2 * ctx_lines + 1);
                ctx.push(v);
            }
        }

        if tags.is_empty() {
            return;
        }

        for (i, line) in haystack.lines().enumerate() {
            for (j, tag) in tags.iter().enumerate() {
                let lower_bound = tag.saturating_sub(ctx_lines);
                let upper_bound = tag + ctx_lines;

                if (i >= lower_bound) && (i <= upper_bound) {
                    let line_as_string = String::from(line);
                    let local_ctx = (i, line_as_string);
                    ctx[j].push(local_ctx);
                }
            }
        }

        for local_ctx in ctx.iter() {
            for &(i, ref line) in local_ctx.iter() {
                let line_num = i + 1;
                println!("{}: {}", line_num, line);
            }
        }
    }

    pub fn p2111() {
        let re = Regex::new("picture").unwrap();

        let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

        for line in quote.lines() {
            let contains_substring = re.find(line);
            match contains_substring {
                Some(_) => println!("{}", line),
                None => (),
            }
        }
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
    
    println!("\n____2.4.6 If, if else, and else: Conditional branching");
    ch2::p246();

    println!("\n____2.4.7 Match: Type-aware pattern matching");
    ch2::p247();

    println!("\n____2.5 Defining functions");
    ch2::p25();

    println!("\n____2.6 Using references");
    ch2::p26();

    // 2.8 Advanced function definitions
    println!("\n____2.8.1 Explicit lifetime annotations");
    ch2::p281();

    println!("\n____2.8.2 Generic functions");
    ch2::p282();

    println!("\n____2.9 Creating grep-lite");
    ch2::p29();

    println!("\n____2.10.1 Arrays");
    ch2::p2101();

    println!("\n____2.10.3 Vectors");
    ch2::p2103();

    println!("\n____2.11.1 Adding support for regular expressions");
    ch2::p2111();
}
