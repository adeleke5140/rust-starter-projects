use std::collections::HashMap;
use std::io;

fn main() {
    let mut f: HashMap<u32, u32> = HashMap::new();
    f.insert(0, 0);
    f.insert(1, 1);

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

        let res = find_fibonacci_sequence(num, &mut f);
        println!("the {}th fibonacci number is {}", num, res);
    }
}

fn find_fibonacci_sequence(n: u32, f: &mut HashMap<u32, u32>) -> u32 {
    let mut sum = 0;
    if n == 0 {
        sum = f[&0];
        return sum;
    } else if n == 1 {
        sum = f[&1];
        return sum;
    } else {
        sum = find_fibonacci_sequence(n - 1, f) + find_fibonacci_sequence(n - 2, f);
        f.insert(n, sum);
        return sum;
    }
}
