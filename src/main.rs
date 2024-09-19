use std::io;
use unicode_segmentation::UnicodeSegmentation; // испольуем крейт для разбиения строки на графемы

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap(); // считываем строку
    let string = input.as_str(); // преобразуем в строковый срез

    let words = string.split_word_bounds().collect::<Vec<&str>>(); // разбиваем строку на слова
    let mut result = String::new();

    for &word in words.iter().rev() {
        // записываем полученные слова в результирующую строку в обратном порядке
        result.push_str(word);
    }
    println!("{}", result);
}
