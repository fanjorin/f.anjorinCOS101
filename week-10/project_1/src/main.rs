// Define the Laptop struct
struct Laptop {
    brand: String,
    price: u32,
}

// function to calculate total cost
fn calculate_total_cost(_laptop: &Laptop, price:u32, quantity:u32) -> u32 {
    price * quantity
}

fn main () {
    // create instances for each laptop brand
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650_000,
    };

    let ibm = Laptop {
         brand: String::from("IBM"),
        price: 755_000,
    };

    let toshiba = Laptop {
         brand: String::from("Toshiba"),
        price: 550_000,

    };

    let dell = Laptop {
         brand: String::from("Dell"),
        price: 850_000,

    };

    // Calculate the total cost of purchasing 3 laptops from each brand
    let total_hp = calculate_total_cost(&hp, 650_000, 3, /* u32 */);
    println!("The total cost for purchasing 3 laptops from hp brand is: ${}", total_hp);

    let total_ibm = calculate_total_cost(&ibm,755_000, 3, /* u32 */);
    println!("The total cost for purchasing 3 laptops from ibm brand is: ${}", total_ibm );

    let total_toshiba = calculate_total_cost(&toshiba, 550_000, 3, /* u32 */);
    println!("The total cost for purchasing 3 laptops from toshiba brand is: ${}", total_toshiba );

    let total_dell = calculate_total_cost(&dell, 850_000, 3, /* u32 */);
    println!("The total cost for purchasing 3 laptops from dell brand is: ${}", total_dell );


}
