use std::io;

fn main() {
    println!("Привет!");
    println!("Данная программа вычисляет вычисляет числа Фибоначчи");
    println!("Пожалуйста введите количество чисел в последовательности:");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Не могу прочитать строку");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Количество чисел должно быть целым положительным числом. Пожалуйста повторите ввод:");
                continue;
            },
        };
    }
}
