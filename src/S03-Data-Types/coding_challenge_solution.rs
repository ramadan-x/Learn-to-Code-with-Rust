fn main() {
    let visual_num: i32 = 1_337;
    let new_visual_num = visual_num as i16;
    let pi: f64 = 3.1234;
    println!("{pi:.3}");
    let with_milk = true;
    let with_sugar = false;
    let is_my_type_of_coffee: bool = with_milk && with_sugar;
    let is_acceptable_coffee: bool = with_milk || with_sugar;
    let array_int: [i8; 4] = [1, 2, 3, 4];

    println!("{array_int:?}");

    let student = ("Ramadan", 18, 3.5);
    let (name, age, gpa) = student;
    println!("Student name {name}, age {age}, GPA {gpa}");
    println!("{student:?}");
}
