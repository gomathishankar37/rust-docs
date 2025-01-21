// shadowing must use let keyword
// shadowed vairables are not mutable variables
// type can be changed in shadowing, with same name
// eg:
// let spaces = "   ";
// let spaces = spaces.len();

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
