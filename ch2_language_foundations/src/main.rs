mod ch2 {
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
        println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);
    
        let one_million: i64 = 1_000_000;
        println!("{}", one_million.pow(2));
    
        let forty_twos = [
            42.0,
            42f32,
            42.0_f32,
        ];
    
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
}
