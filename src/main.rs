use std::io;

fn main() {
    let mut temp = String::new();

    println!("Please type in your temperature in celcius:");
    io::stdin()
        .read_line(&mut temp)
        .expect("Could not read line");

    if temp.trim().contains("C") {
        temp.retain(|c| c != 'C');
        let temp: i32 = String::from(temp)
            .trim()
            .parse()
            .expect("Could not read temperature value");
        convert_to_farenheit(temp);
    } else if temp.contains("F") {
        temp.retain(|c| c != 'F');
        let temp: f32 = String::from(temp)
            .trim()
            .parse()
            .expect("Could not read temperature value");
        convert_to_celcius(temp);
    } else {
        eprintln!("Cannot convert temperature without unit");
    }
}

fn convert_to_farenheit(num: i32) {
    let num = (num as f32 * 1.8) + 32 as f32;
    println!("the temp in Farenheit is {}F", num);
}

fn convert_to_celcius(num: f32) {
    let num = (num - 32 as f32) / 1.8;
    println!("the temp in Celcius is {}C", num);
}
