use std::io;
mod utils;

const IO_STDIN_ERROR_MESSAGE: &str = "Couldn't get your Input";

fn main() {
    let mut temp_unit = String::new();

    // Get Temp Unit
    loop {
        println!("Convert from Celsius or Fahrenheit?");
        println!("Type \"C\" or \"F\" to choose..");
        io::stdin()
            .read_line(&mut temp_unit)
            .expect(IO_STDIN_ERROR_MESSAGE);

        temp_unit = temp_unit.trim().to_lowercase();

        if temp_unit == "c" || temp_unit == "f" {
            break;
        } else {
            // Reset Input Value
            temp_unit = "".to_string();
            println!("Invalid Temperature Unit 👎, Try again! ");
        }
    }

    let degree;

    loop {
        println!(
            "Enter the degree in {}°",
            utils::string_format::capitalize(temp_unit.to_owned())
        );
        let mut degree_string = String::new();

        io::stdin()
            .read_line(&mut degree_string)
            .expect(IO_STDIN_ERROR_MESSAGE);

        let degree_num: f64 = match degree_string.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Please Enter a number!");
                continue;
            }
        };
        degree = degree_num;

        break;
    }

    let result: f64;

    if temp_unit == "c" {
        result = (degree * 9.0 / 5.0) + 32.0;
    } else {
        result = (degree - 32.0) * 5.0 / 9.0;
    }

    let opposite_temp_unit = if temp_unit == "c" { "f" } else { "c" };

    println!(
        "{}°{} = {:.1}°{}",
        degree,
        utils::string_format::capitalize(temp_unit),
        result,
        utils::string_format::capitalize(opposite_temp_unit.to_owned())
    );
}
