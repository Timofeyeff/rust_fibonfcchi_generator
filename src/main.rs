use std::io;
use std::f64;
use std::time::Instant;

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
        println!("");
        println!("Вычисляю число по замкнутой формуле");
        if guess < 94 {
            let sys_time = Instant::now();
            let fib = fib_close(guess);
            let difference = sys_time.elapsed();
            println!("Время выполнения функции: {:?}", difference);
            println!("Полученное значение: {}", fib);
        } else {
            println!("Не хочу считать - разрядности не хватает, хочу печеньку!");
        }
        println!("");
        
        println!("Вычисляю число методом классичекой рекурсии");
        if guess < 45 {
            let sys_time = Instant::now();
            let fib = fib_recursive(guess);
            let difference = sys_time.elapsed();
            println!("Время выполнения функции: {:?}", difference);
            println!("Полученное значение: {}", fib);
        } else {
            println!("Не хочу считать - слишком долго, хочу печеньку!");
        }
        println!("");
        
        println!("Вычисляю число методом хвостовой рекурсии");
        if guess < 93 {
            let sys_time = Instant::now();
            let fib = fib_tail_recursive(guess);
            let difference = sys_time.elapsed();
            println!("Время выполнения функции: {:?}", difference);
            println!("Полученное значение: {}", fib);
        } else {
            println!("Не хочу считать - буфер переполню, хочу печеньку!");
        }
        println!("");
        
        println!("Вычисляю число методом динамического программирования");
        //let mut vec_fib: Vec<u64> = vec![0, 1];
        if guess < 93 {
            let sys_time = Instant::now();
            let fib = fib_memory(guess);
            let difference = sys_time.elapsed();
            println!("Время выполнения функции: {:?}", difference);
            println!("Полученное значение: {}", fib);
        } else {
            println!("Не хочу считать - буфер переполню, хочу печеньку!");
        }
        println!("");
        
        println!("Вычисляю число методом матричной алгебры");
        break;
    }
}

//Замкнутая формула
fn fib_close (n: u64) -> u64 {
    let sqrt5: f64 = 5f64.sqrt();
    let phi: f64 = (sqrt5 + 1f64) / 2f64;
    (phi.powf(n as f64) / sqrt5 + 0.5) as u64
}

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

//Вычисление с запоминанием
fn fib_memory (n: u64) -> u64 {
    let (mut cur, mut next) = (0u64, 1u64);
    for _ in 0u64..n {
        let tmp = cur + next;
        cur = next;
        next = tmp;
    }
    cur
}
