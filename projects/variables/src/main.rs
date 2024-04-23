fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");

    let x = 2.0; //f64
    let y: f32 = 3.0; //f32

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let q = tup.0;
    println!("{q} {q}");

    let g: [i32; 5] = [1,2,3,4,5];
    //println!("{a}");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");


}