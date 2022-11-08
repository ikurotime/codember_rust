use std::fs;

const REQUIRED_FIELDS: [&str; 6] = ["usr", "eme", "psw", "age", "loc", "fll"];
struct User {
    usr: String,
}
fn main() {
    println!("In file {}", "users.txt");
    let contents = fs::read_to_string("users.txt").expect("Should have been able to read the file");
    let binding = contents
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.replace("\n", " "))
        .collect::<Vec<String>>();
    let users = binding
        .iter()
        .map(|user_array| {
            let mut user_map = std::collections::HashMap::new();
            user_array.split(" ").for_each(|user| {
                let user = user.split(":").collect::<Vec<&str>>(); //split user by : and push to hash map
                user_map.insert(user[0], user[1]);
            });
            user_map //return hash map
        })
        .collect::<Vec<std::collections::HashMap<&str, &str>>>();
    let valid_users = users
        .iter()
        .filter(|user| REQUIRED_FIELDS.iter().all(|field| user.contains_key(field))) // check if all required fields are present
        .map(|user| User {
            usr: user["usr"].to_string(),
        })
        .collect::<Vec<User>>();
    println!(
        "Last valid username: {}",
        valid_users[valid_users.len() - 1].usr
    );
    println!("Number of valid users: {}", valid_users.len());
}
