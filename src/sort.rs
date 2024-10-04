use std::cmp::Ordering;
use std::collections::HashMap;

use crate::cli::Cli;

pub fn sort<'a>(lines: &mut Vec<&'a str>, cli: &Cli) {
    lines.sort_by(|a, b| {
        let a_columns: Vec<&str> = a.split_whitespace().collect();
        let b_columns: Vec<&str> = b.split_whitespace().collect();

        let column_index = cli.key - 1;

        let a_column = a_columns.get(column_index);
        let b_column = b_columns.get(column_index);
        match (a_column, b_column) {
            (Some(a_value), Some(b_value)) => if cli.numeric_sort {
                numeric_sort_comparator(a, b, a_value, b_value, column_index)
            } else if cli.month_sort {
                month_sort_comparator(a, b, column_index)
            } else if cli.human_numeric_sort {
                human_numeric_sort_comparator(a, b, a_value, b_value)
            } else {
                lexicographical_sort_comparator(a, b, column_index)
            },
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => a.cmp(b)
        }
    });
}

fn lexicographical_sort_comparator(a: &str, b: &str, column_index: usize) -> std::cmp::Ordering {
    a.split_at(find_index_of_key_word(a, column_index).unwrap()).1
        .cmp(b.split_at(find_index_of_key_word(b, column_index).unwrap()).1)
}

fn numeric_sort_comparator(a: &str, b: &str, a_value: &str, b_value: &str, column_index: usize) -> Ordering {
    // Пытаемся распарсить обе строки в числа
    let a_parsed = a_value.parse::<i32>();
    let b_parsed = b_value.parse::<i32>();

    match (a_parsed, b_parsed) {
        // Если обе строки числа, сортируем их как числа
        (Ok(a_num), Ok(b_num)) => if a_num == b_num {
            lexicographical_sort_comparator(a, b, column_index)
        } else {
            a_num.cmp(&b_num)
        },
        // Если одна строка число, а другая нет, считаем, что число "меньше"
        (Ok(num), Err(_)) | (Err(_), Ok(num)) => if num <= 0 { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater },
        // Если обе строки не числа, сортируем лексикографически
        (Err(_), Err(_)) => a_value.cmp(b_value),
    }
}

fn month_sort_comparator(a: &str, b: &str, column_index: usize) -> std::cmp::Ordering {
    let month_order: HashMap<&str, usize> = [
        ("January", 1),
        ("February", 2),
        ("March", 3),
        ("April", 4),
        ("May", 5),
        ("June", 6),
        ("July", 7),
        ("August", 8),
        ("September", 9),
        ("October", 10),
        ("November", 11),
        ("December", 12),
    ].iter().cloned().collect();

    let a_is_month = month_order.get(a);
    let b_is_month = month_order.get(b);

    match (a_is_month, b_is_month) {
        // Если обе строки - месяцы, сортируем по порядковому номеру
        (Some(a_num), Some(b_num)) => if a_num == b_num {
            lexicographical_sort_comparator(a, b, column_index)
        } else {
            a_num.cmp(b_num) 
        },
        // Если одна строка - месяц, а другая нет, месяц "меньше"
        (Some(_), None) => std::cmp::Ordering::Greater,
        (None, Some(_)) => std::cmp::Ordering::Less,
        // Если обе строки не месяцы, сортируем лексикографически
        (None, None) => a.cmp(b),
    }
}

fn human_numeric_sort_comparator(a: &str, b: &str, a_value: &str, b_value: &str) -> std::cmp::Ordering {
    // Определяем суффиксы и их числовые значения
    let suffixes: Vec<(&str, f64)> = vec![
        ("", 1.0),    // Нет суффикса
        ("K", 1e3),   // Кило
        ("M", 1e6),   // Мега
        ("G", 1e9),   // Гига
        ("T", 1e12),  // Тера
        ("P", 1e15),  // Пета
        ("E", 1e18),  // Экса
        ("Z", 1e21),  // Зетта
        ("Y", 1e24),  // Йотта
    ];

    // Создаем хэш-таблицу для суффиксов
    let suffix_map: HashMap<&str, f64> = suffixes.into_iter().collect();

    let (num_a, suffix_a) = parse_human_numeric(a_value, &suffix_map);
    let (num_b, suffix_b) = parse_human_numeric(b_value, &suffix_map);

    // Сравниваем числовые значения с учетом суффиксов
    match (num_a * suffix_a).partial_cmp(&(num_b * suffix_b)) {
        Some(Ordering::Equal) => a.cmp(b), // Если численные значения равны, сравниваем лексикографически
        Some(ordering) => ordering,
        None => Ordering::Equal,
    }
}

// Функция для разбора строки на числовую часть и суффикс
fn parse_human_numeric(s: &str, suffix_map: &std::collections::HashMap<&str, f64>) -> (f64, f64) {
    let mut num_part = String::new();
    let mut suffix_part = String::new();

    // Разделяем строку на числовую и суффикс
    for c in s.chars() {
        if c.is_digit(10) || c == '.' || c == '-' {
            num_part.push(c);
        } else {
            suffix_part.push(c);
        }
    }

    // Преобразуем числовую часть в число
    let num_value: f64 = num_part.parse().unwrap_or(0.0);
    // Получаем коэффициент для суффикса (если нет суффикса, коэффициент = 1.0)
    let suffix_value = suffix_map.get(suffix_part.as_str()).cloned().unwrap_or(1.0);

    (num_value, suffix_value)
}

fn find_index_of_key_word(line: &str, key: usize) -> Option<usize> {
    let mut is_word = false;
    let mut key = key;
    for (i, c) in line.chars().enumerate() {
        if c.is_whitespace() {
            is_word = false;
        } else if !is_word {
            is_word = true;
            if key == 0 {
                return Some(i);
            }
            key -= 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_sort_comparator() {
        assert_eq!(numeric_sort_comparator("10", "2", "10", "2", 0), Ordering::Greater);
    }

    #[test]
    fn test_find_index_of_key_word_middle() {
        assert_eq!(find_index_of_key_word("a b c", 1), Some(2));
    }

    #[test]
    fn test_find_index_of_key_word_beginning() {
        assert_eq!(find_index_of_key_word("a b c", 0), Some(0));
    }

    #[test]
    fn test_compare_strings_with_column_0() {
        assert_eq!(lexicographical_sort_comparator("2 a", "2 b", 0), Ordering::Less);
    }

    #[test]
    fn test_compare_strings_with_column_1() {
        assert_eq!(lexicographical_sort_comparator("2    a", "2  b", 1), Ordering::Less);
    }

    #[test]
    fn test_compare_strings_with_column_when_no_column() {
        assert_eq!(lexicographical_sort_comparator("2", "2 b", 0), Ordering::Less);}
}