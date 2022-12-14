use std::fs;
use std::vec;

const REQUIRED_FIELDS: [&str; 6] = ["usr", "age", "eme", "psw", "loc", "fll"];

fn main() {
    let file_path = "./users.txt";

    let contents = fs::read_to_string(file_path)
        .expect("failed to read file");

    let users = String::from(contents);

    let mut users_data: Vec<String> = vec![];

    let mut user_str =  String::from("");
    for user in users.lines().enumerate() {
        let content = user.1;
        if content != "" {
            user_str.push_str(&content);
        } else {
            users_data.push(user_str);
            user_str = String::from("")
        }
    }

    let data: Vec<String> = users_data.into_iter()
                                        .filter(|u| have_fields(u))
                                        .collect();

    println!("Number of users -> {:?}", data.len());
    println!("Last user valid -> {:?}", data[ data.len() -1 ]);

}


fn have_fields(data: &String) -> bool {
    data.contains(REQUIRED_FIELDS[0]) &&  data.contains(REQUIRED_FIELDS[1]) &&  data.contains(REQUIRED_FIELDS[2]) &&  data.contains(REQUIRED_FIELDS[3]) &&  data.contains(REQUIRED_FIELDS[4]) &&  data.contains(REQUIRED_FIELDS[5])
}