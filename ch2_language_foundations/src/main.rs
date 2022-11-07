mod ch2 {
    pub fn p221_variables_functions() {
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
}

fn main() {
    println!("Defining variables and calling functions");
    ch2::p221_variables_functions();


}
