use std::io::{BufRead, BufReader};
use std::fs::File;
use std::fs;

fn reverse_substring(s: &mut String, a: usize, b: usize) {
    let mut left = a;
    let mut right = b;

    // Проверяем, что индексы находятся в пределах строки
    if left >= s.len() || right >= s.len() || left >= right {
        return; // Неправильные индексы, ничего не делаем
    }

    while left < right {
        // Меняем местами символы
        swap(s, left, right);
        left += 1;
        right -= 1;
    }
}

fn swap(s: &mut String, a: usize, b: usize) {
    let mut chars: Vec<char> = s.chars().collect(); // Преобразуем строку в вектор символов
    if a < chars.len() && b < chars.len() { // Проверяем, что индексы находятся в пределах
        let temp = chars[a];
        chars[a] = chars[b];
        chars[b] = temp;
        
        *s = chars.into_iter().collect(); // Преобразуем вектор обратно в строку
    }
}

fn read_my_file() -> (usize, usize, String){
    let reader = BufReader::new(File::open("file.txt").expect("Cannot open file.txt"));
    let mut v: Vec<String> = Vec::new();
    for line in reader.lines() {
        match line {
            Err(_e) => panic!("Ошибка чтения"),
            Ok(f) => v.push(f),
        }
    }
    let v1: Vec<&str> = v[0].split(' ').collect();
    let tup:(usize, usize, String) = (
        v1[0].parse::<usize>().unwrap(),
        v1[1].parse::<usize>().unwrap(),
        v[1].to_string()
    );
    return tup;

}

fn write_data_to_file(answer: String){
    let _ = fs::write("answer.txt", answer);
}

fn main() {
    let data:(usize, usize, String) = read_my_file();
    let mut word = data.2;

    println!("Исходная строка: {}", word);
    reverse_substring(&mut word, data.0, data.1);
    println!("Измененная строка: {}", word);
    write_data_to_file(word);
}
