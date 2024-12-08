fn find_most_experienced (developers: Vec<(&str, u8)>) -> (&str, u8) {
    let mut max_experience =0;
    let mut most_experienced = ("", 0);

    for developer in developers {
        if developer.1 > max_experience {
            max_experience = developer.1;
            most_experienced = developer;

        }
    }
    most_experienced
}

fn main() {
    let developers = vec![
    ("Alice", 5),
    ("Bob", 8),
    ("Feyi", 15),
    ("Diana", 7),
    ];
    let most_experienced = find_most_experienced(developers);
    println!("The developer with the highesr experience is {} with {} years of experience.", most_experienced.0,most_experienced.1 );
}
