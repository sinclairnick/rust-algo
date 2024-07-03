const OPEN_BRACKET: &char = &'(';
const CLOSED_BRACKET: &char = &')';

pub fn has_balanced_parens(val: &str) -> bool {
    let mut remaining = String::new();

    println!("{val}");

    if val.len() == 0 {
        return true;
    }

    let mut i = 0;
    let chars: Vec<_> = val.chars().collect();

    while i < val.len() {
        let char = chars[i];
        let mut next_char = char::default();

        if val.len() > i + 1 {
            next_char = chars[i + 1];
        }

        println!("Char: '{char}', Next: '{next_char}'");

        // Skip over the complete brackets
        if char.eq(&OPEN_BRACKET) && next_char.eq(&CLOSED_BRACKET) {
            println!("MATCH!");
            i += 2;
            continue;
        }

        // Otherwise append to remaining string
        remaining.push(char);
        i += 1
    }

    println!("R: {remaining}");

    if remaining.len() != val.len() {
        return has_balanced_parens(&remaining);
    }

    return false;
}

#[cfg(test)]
mod test {
    use super::has_balanced_parens;

    #[test]
    fn test_simple() {
        let result = has_balanced_parens("()");

        assert_eq!(result, true);
    }

    #[test]
    fn test_harder() {
        let result = has_balanced_parens("()()");

        assert_eq!(result, true);
    }

    #[test]
    fn test_hardest() {
        let result = has_balanced_parens("(()())()");

        assert_eq!(result, true);
    }

    #[test]
    fn test_negative_simple() {
        let result = has_balanced_parens("(");

        assert_eq!(result, false);
    }

    #[test]
    fn test_negative_harder() {
        let result = has_balanced_parens("()(");

        assert_eq!(result, false);
    }

    #[test]
    fn test_negative_hardest() {
        let result = has_balanced_parens("(()()()");

        assert_eq!(result, false);
    }
}
