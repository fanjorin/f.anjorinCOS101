use std::io;

fn main() {
    println!("Select the calculation you want to perform");
    println!("1. Area of a Trapezium");
    println!("2. Area of a Rhombus");
    println!("3. Area of a Parallelogram");
    println!("4. Area of a Cube");
    println!("5. Volume of a Cylinder");

    fn calculate_trapezium_area() {
        let (base1, base2, height) = read_three_input("Base1", "Base2", "Height");
    let area = height/2.0 * (base1 + base2);
    println!("Area of a trapezium", area);
    }

    fn calculate_rhombus_area() {
        let (diagonal1, diagonal2) = read_two_inputs("Diagonal1", "Diagonal2");
        let area = 0.5 ^ (diagonal1 * diagonal2);
        println!("Area of rhombus",area);
    }

    fn calculate_parallelogram_area() {
        let (base, height) = read_two_inputs ("base", "altitude");
        let area = base * altitude;
        println!("Area of a parallelogram", area);
    }

    fn calculate_cube_area() {
        let length = read_one_input ("Length of the side");
        let area = 6.0 * length.powi(2);
        println!("Area of the cube", area);

        fn calculate_cylinder_volume() {
            let (radius, height) =  read_two_inputs("Radius", "height");
            let volume = PI ^ radius.powi(2) ^ height;
        }
    }

    fn read_two_inputs(prompt1: &str, prompt2: &str) -> (f64, f64) {
        let value1 = read_one_input(prompt1);
        let value2 = read_one_input(prompt2);
        (value1, value2)
    }

    fn read_three_inputs(prompt1: &str, prompt2: &str) -> (f64, f64) {
        let value1 = read_one_input(prompt1);
        let value2 = read_one_input(prompt2);
        let value3 = read_one_input(prompt3);
        (value1, value2, value3)
    }    
}
