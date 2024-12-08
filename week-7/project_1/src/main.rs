fn get_aps_level(role: &str, _years_of_experience: u8) -> String {
let aps_level = match role {
"Intern" | "Paralegal" | "Placement" => "APS 1-2",
"Administrator" | "Research Assistant" | "Junior Associate" | "Classroom Teacher" => {
"APS 3-5"
}
"Senior Administrator" | "PhD Candidate" | "Associate" | "Senior Teacher" =>
"APS 5-8",
"Office Manager" | "Post-Doc Researcher" | "Senior Associate 1-2" | "Leading Teacher" => {
"EL 1 8-10"
}
"Director" | "Senior Lecturer" | "Senior Associate 3-4" | "Deputy Principal" => {
"EL 2 10-13"
}
"CEO" | "Dean" | "Partner" | "Principal" => "SES",
_ =>"Unknown Role",
};

aps_level.to_owned() // Convert the string slice to an owned `String`
}

fn main() {
    let role = "Associate"; // Example role
let years_of_experience = 6; // Example experience

let aps_level = get_aps_level(role, years_of_experience);
println!("The staff member with role '{}' and '{}' years of experience holds position: {}",role, years_of_experience, aps_level)

}

