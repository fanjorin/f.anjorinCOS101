use std::io;

fn main() {
    // Get input for experience
    let mut experience = String::new();
    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut experience).expect("Failed to read input");

    // Get input for age 
    let mut age = String::new();
    println!("Enter the age of the employee:");
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age: u32 = age.trim().parse().expect("Invalid input");

    // Determine the incentive
    let incentive = if experience == "yes" {
        if age >= 40 {
            1_560_000
            } else if age >= 30 && age < 40 {
                1_480_000
            } else if age < 28 {
                1_300_000
            } else {
                0 // No specific incentive provided for other ages in he experienced category
            }
             } else {
                100_000  // Incentive for inexperienced employess
             }
            }
            }
        }
        // Print the incentive
        println!("The annual incentive for the employees is: N{}", incentive);
    }

}
