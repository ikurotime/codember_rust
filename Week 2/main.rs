use std::fs;


fn main (){
  println!("In file encrypted.txt");
  let contents = fs::read_to_string("encrypted.txt").expect("Shou>ld have been able to read the file"); //read file
  println!("The encrypted message is: {}", contents); //print encrypted message
  let mut decrypted = Vec::new(); //create new empty array
  let mut current_ascii: String = "".to_string(); // create new empty string to store current ascii value
  for letter in contents.chars() { //iterate over each letter in file
    if letter == ' ' {
      decrypted.push(letter); //if letter is space push to array
    }else{
      current_ascii.push_str(&letter.to_string()); // we store the numbers in a string
      if letter.is_numeric(){
        if current_ascii.parse::<i32>().unwrap() >= 97 && current_ascii.parse::<i32>().unwrap() <= 122{ 
          /*if the current ascii value is between 97 and 122 (an ascii letter)
          we push the letter to the array */
          decrypted.push(current_ascii.parse::<u8>().unwrap() as char); //we convert the ascii value to a char and push to array
          current_ascii = "".to_string(); // we clean the string
        }
      }
    }
  }

  println!("Decrypted: {}", decrypted.iter().collect::<String>()); // The final message
}