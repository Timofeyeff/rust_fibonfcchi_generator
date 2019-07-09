use std::io;

fn main() {
    println!("Привет!");
    println!("Данная программа вычисляет вычисляет числа Фибоначчи");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .except("Не могу прочитать строку");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Пожалуйста введите целое положительное число:");
                continue;
            },
        };
    }
}
