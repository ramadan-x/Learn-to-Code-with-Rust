fn main() {
    fn apply_to_jobs(number: i8, title: &str) {
        println!("I'm applying to: {}, Title: {}", number, title);
    }
    apply_to_jobs(35, "Rust Developer");

    fn is_even(num: i8) -> bool {
        num % 2 == 0
    }
    println!("{}", is_even(4));
    println!("{}", is_even(5));

    fn alphabets(text: &str) -> (bool, bool) {
        (text.contains('a'), text.contains('z'))
    }
    println!("{:?}", alphabets("aardvark")); // -> (true, false)
    println!("{:?}", alphabets("zoology")); // -> (false, true)
    println!("{:?}", alphabets("zebra")); // -> (true, true)
}
