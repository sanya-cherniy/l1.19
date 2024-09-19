use std::io;
use unicode_segmentation::UnicodeSegmentation; // испольуем крейт для разбиения строки на графемы

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input); // считываем строку
    let string = input.as_str(); // преобразуем в строковый срез
                                 // let graphemes = UnicodeSegmentation::graphemes(string, true).collect::<Vec<&str>>(); // разбиваем строку на графемы
    let words = string.split_word_bounds().collect::<Vec<&str>>();
    let mut result = String::new();
    for &word in words.iter().rev() {
        // записываем полученные графемы в результирующую строку в обратном порядке
        result.push_str(word);
    }
    println!("{}", result);
}
