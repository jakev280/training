use std::fs;
// use std::io::Error;

fn extract_errors(text: &str) -> Vec<&str> {
    
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }

    results
}

fn main() {

    // Other option -> unwrap(), expect() Want to crash on Err, good for quick debug
    // Other option -> Try operator (no workaround)

    match fs::read_to_string("logs.txt") {
        Ok(text_that_was_read) => {
            let error_logs= extract_errors(text_that_was_read.as_str());
            
            match fs::write("errors.txt", error_logs.join("\n")) {
                Ok(..) => println!("wrote errors.txt"),
                Err(reason_write_failed) => {
                    println!("Writing of errors.txt failed: {}", reason_write_failed);
                }
            }
        }
        Err(why_this_failed) => {
            println!("Failed to read file: {}", why_this_failed);
        }
    } 

    // match divide(5.0, 0.0) {
    //     Ok(result_of_division) => {
    //         println!("{}", result_of_division);
    //     }
    //     Err(what_went_wrong) => {
    //         println!("{}", what_went_wrong);
    //     }
    // }

    // match validate_email(String::from("jake@gmail.com")) {
    //     Ok(..) => println!("email is valid"),
    //     Err(reason_for_failed_validation) => {
    //         println!("{}", reason_for_failed_validation)
    //     }
    // }
}

// fn validate_email(email: String) -> Result<(), Error> {
//     if email.contains("@") {
//         //Success!
//         Ok(())
//     }
//     else {
//         Err(Error::other("email must contain an @"))
//     }
// }

// fn divide(a: f64, b: f64) -> Result<f64, Error> {
//     if b== 0.0 {
//         Err(Error::other("Can't didvie by 0"))
//     } else {
//         Ok(a / b)
//     }
// }