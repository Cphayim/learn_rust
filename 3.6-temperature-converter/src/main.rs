use std::{io, process::exit};

const CELSIUS_TO_FAHRENHEIT: u8 = 1;
const FAHRENHEIT_TO_CELSIUS: u8 = 2;
const EXIT: u8 = 3;

fn main() {
    println!("温度转换器");
    println!("1. 摄氏度转华氏度");
    println!("2. 华氏度转摄氏度");
    println!("3. 退出");

    let mut command = String::new();

    io::stdin().read_line(&mut command).expect("读取命令失败");

    let command: u8 = match command.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("输入的不是有效数字");
            return;
        }
    };

    if command == CELSIUS_TO_FAHRENHEIT || command == FAHRENHEIT_TO_CELSIUS {
        interactive(command);
    } else if command == EXIT {
        exit(0);
    }
}

fn interactive(mode: u8) {
    if mode != CELSIUS_TO_FAHRENHEIT && mode != FAHRENHEIT_TO_CELSIUS {
        return;
    }

    let source_text = if mode == CELSIUS_TO_FAHRENHEIT {
        "摄氏度"
    } else {
        "华氏度"
    };
    let target_text = if mode == CELSIUS_TO_FAHRENHEIT {
        "华氏度"
    } else {
        "摄氏度"
    };

    println!("请输入{source_text}: ");

    let mut source = String::new();

    io::stdin().read_line(&mut source).expect("读取温度失败");

    let source: f64 = match source.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("输入的温度不是有效数字");
            return;
        }
    };

    let target = if mode == CELSIUS_TO_FAHRENHEIT {
        celsius_to_fahrenheit(source)
    } else {
        fahrenheit_to_celsius(source)
    };

    println!("转换后的{target_text}为: {target}");
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}
