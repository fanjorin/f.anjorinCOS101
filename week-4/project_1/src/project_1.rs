use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    // Getting inputs from the user
    println!("Enter the value of a:");
    io::stdin().read_line(&mut a).expect("Failed to read input");

    println!("Enter the value of b:");
    io::stdin().read_line(&mut b).expect("Failed to read input");

    println!("Enter the value of c:");
    io::stdin().read_line(&mut c).expect("Failed to read input");

    // Converting input strings to f64
    let a:f64 = a.trim().parse().expect("Invalid input");
    let b:f64 = b.trim().parse().expect("Invalid input");
    let c:f64 = c.trim().parse().expect("Invalid input");

    //Calculating the discriminant
    let discriminant = b * b - 4.0 * a * c;

    //Checking the nature of the roots based on the discriminant
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);

        println!("The roots are real and distinct: root1 = {}", root1, root2);

        else if discriminant ==0.0 {
            let root = -b / (2.0 * a);
        } 
        println!("The root is real and repeated: root = {}",root);
        else {
            println!("There are no real root");
        }
    }


}
