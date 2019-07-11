use std::io;
use std::time::SystemTime;

fn main() {
    println!("Привет!");
    println!("Данная программа вычисляет вычисляет числа Фибоначчи");
    println!("Пожалуйста введите количество чисел в последовательности:");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Не могу прочитать строку");
        let guess: u64 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Количество чисел должно быть целым положительным числом. Пожалуйста повторите ввод:");
                continue;
            },
        };
        println!("Вычисляю число по замкнутой формуле");
        
        println!("Вычисляю число методом классичекой рекурсии");
        let sys_time = SystemTime::now();
        let fib = fib_recursive(guess);
        let difference = sys_time.duration_since(sys_time)
                         .expect("SystemTime::duration_since failed");
        println!("Время выполнения функции: {:?}", difference);
        println!("Полученное значение: {}", fib);
        println!("Вычисляю число методом хвостовой рекурсии");
        println!("Полученное значение: {}", fib_tail_recursive(guess));
        println!("Вычисляю число методом запоминания");
        println!("Вычисляю число методом динамического программирования");
        println!("Вычисляю число методом матричной алгебры");
        break;
    }
}

//fn fib1 (n: u64) -> u64 {
//    const SQRT5: f64 = sqrt(5);
//    const PHI: f64 = (SQRT5 + 1) / 2;
//    (PHI * n
//}
//
//Классическая рекурсивная версия
//без хвостовой рекурсии
fn fib_recursive (n: u64) -> u64 {
    match n {
        0 | 1 => n,
        n => fib_recursive(n - 1) + fib_recursive(n - 2),
    }
}

//с хвостовой рекурсией
fn fib_tail_recursive (n: u64) -> u64 {
    fn in_fib (n: u64, current: u64, next: u64) -> u64 {
        match n {
            0 => current,
            n => in_fib (n-1, next, current + next),
        }
    }
    in_fib (n, 0, 1)
}
