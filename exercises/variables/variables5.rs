// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)


fn main() {
    let mut number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    number = "3";
    let int_num:i32 = number.parse().unwrap();
    println!("Number plus two is : {}", int_num + 2);
}
