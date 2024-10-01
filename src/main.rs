fn main() {
    println!("{}", unpack_string("qwe\\45e\\6\\\\\\5"));
}

pub fn unpack_string(input: &str) -> String {
    let mut times: i32 = 0;
    let mut res: String = String::new();
    let mut curr: Option<char> = None;
    let mut esc = false;
    for c in input.chars() {
        if c == '\\' && !esc {
            esc = true;
            continue;
        }
        if c.is_digit(10) && c.ne(&'0') && !esc {
            times = times * 10 + c.to_digit(10).unwrap() as i32;
        } else {
            for _ in 0..times - 1 {
                res.push(curr.unwrap());
            }
            res.push(c);
            curr.replace(c);
            times = 0;
        }
        esc = false;
    }

    if curr.is_some() {    
        for _ in 0..times - 1 {
            res.push(curr.unwrap());
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unpack_string_regular() {
        assert_eq!(unpack_string("a4bc2d5e"), "aaaabccddddde");
    }

    #[test]
    fn test_unpack_string_regular_with_escapes() {
        assert_eq!(unpack_string("qwe\\45e\\6\\\\\\5"), "qwe44444e6\\5");
    }

    #[test]
    fn test_unpack_string_no_numbers() {
        assert_eq!(unpack_string("abcd"), "abcd");
    }

    #[test]
    fn test_unpack_string_only_numbers() {
        assert_eq!(unpack_string("45"), "");
    }

    #[test]
    fn test_unpack_string_empty_string() {
        assert_eq!(unpack_string(""), "");
    }

    #[test]
    fn test_unpack_string_number_in_the_end() {
        assert_eq!(unpack_string("qwe5"), "qweeeee");
    }

    #[test]
    fn test_unpack_string_character_in_the_end() {
        assert_eq!(unpack_string("qwe5d"), "qweeeeed");
    }

    #[test]
    fn test_unpack_string_esc_packed_number() {
        assert_eq!(unpack_string("qwe\\45"), "qwe44444");
    }


    #[test]
    fn test_unpack_string_esc_number() {
        assert_eq!(unpack_string("qwe\\4\\5"), "qwe45");
    }


    #[test]
    fn test_unpack_string_spaces() {
        assert_eq!(unpack_string("      "), "      ");
    }

    #[test]
    fn test_unpack_string_esc_backslash() {
        assert_eq!(unpack_string("\\\\"), "\\");
    }

    #[test]
    fn test_unpack_string_esc_packed_backslash() {
        assert_eq!(unpack_string("qwe\\\\5"), "qwe\\\\\\\\\\");
    }

    #[test]
    fn test_unpack_string_number_only_backslash() {
        assert_eq!(unpack_string("\\"), "");
    }

    // decided that in that case acception zero as just a character and not a number is the best option
    // beacause we pack string to save memory and in the case "a0b" -> "b" we don't do that
    #[test]
    fn test_unpack_string_zero() {
        assert_eq!(unpack_string("a0b"), "a0b");
    }
}