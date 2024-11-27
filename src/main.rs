use std::io::{self, Write};

fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let mut a: u64 = 0;
        let mut b: u64 = 1;
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        b
    }
}

fn main() {
    println!("Program Fibonacci ke-n");
    print!("Masukkan nilai n: ");
    io::stdout().flush().expect("Gagal meng-*flush* stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Gagal membaca input");

    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Harap masukkan angka positif.");
            return;
        }
    };

    let result = fibonacci(n);
    println!("Bilangan Fibonacci ke-{n} adalah {result}");
}
