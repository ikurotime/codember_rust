use std::fs;

fn main() {
    println!("In file {}", "users.txt");
    let contents = fs::read_to_string("users.txt")
        .expect("Should have been able to read the file");
        //Create new empty array
    let mut users: Vec<String> = Vec::new();
    contents.lines().for_each(|line| {
        //if line is empty sustitute with "|" and push to array, else push line to array
        if line.trim().is_empty() {
          users.push("|".to_string());
        } else {
          users.push(line.to_string());
        }
    });
    let binding = users.join(" ");
    let users = binding.split("|").collect::<Vec<&str>>();    
    let mut valid_users: Vec<String> = Vec::new();
    //Iterate over users array and push to valid_users array if it fits the requirements
    for user in users {
      if user.contains("usr:") && user.contains("eme:") && user.contains("psw:") && user.contains("age:") && user.contains("loc:") && user.contains("fll:")  {
        valid_users.push(user.to_string());
      }
    }
    println!("Last valid user: {}", valid_users[valid_users.len() - 1].to_string());
    println!("Number of valid users: {}", valid_users.len());
    
}