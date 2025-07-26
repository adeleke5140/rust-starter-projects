use std::io;

fn main() {
    loop {
        println!("Enter the number to get its fibonacci value");

        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("could not read number");

        let num: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(error) => panic!("Problem parsing number: {error:?}"),
        };

        let res = find_fibonacci_sequence(num);
        println!("the {}th fibonacci number is {}", num, res);
    }
}

fn find_fibonacci_sequence(n: u32) -> u32 {
    let mut sum = 0;
    if n == 0 {
        sum += 0;
        return sum;
    } else if n == 1 {
        sum+=1;
        return sum;
    } else {
        sum = find_fibonacci_sequence(n - 1) + find_fibonacci_sequence(n - 2);
        return sum;
    }
}
