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
}

fn main() {
    println!("____2.2.1 Defining variables and calling functions");
    ch2::p221();

    println!("____2.3.1 Integers and decimal (floating-point) numbers");
    ch2::p231();
}
