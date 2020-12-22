mod helpers;

fn main() {
    let filename: &str = "day18.txt";
    let input = &helpers::input_helpers::read_input(&filename).unwrap();
    task_1(input);
    task_2(input);
}

fn task_1(input: &[String]) -> i64 {
    let result = input.iter().map(task_1_internal).sum();
    println!("Task 1: {}", result);
    result
}

fn task_2(input: &[String]) -> i64 {
    let result = input.iter().map(task_2_internal).sum();
    println!("Task 2: {}", result);
    result
}

fn task_1_internal(input: &String) -> i64 {
    ParseItem::parse_1(&input.replace(" ", "").chars().collect::<Vec<char>>()).evaluate()
}

fn task_2_internal(input: &String) -> i64 {
    parse(&tokenize(&input)).evaluate()
}

fn tokenize(input: &String) -> Vec<Token> {
    let chars: Vec<char> = input.replace(" ", "").chars().collect();

    let mut results = Vec::new();
    let mut idx: usize = 0;
    while idx < chars.len() {
        let mut token_len = 1;
        while idx+token_len < chars.len() && chars[idx].is_digit(10) && chars[idx+token_len].is_digit(10) {
            token_len += 1;
        }

        results.push(match chars[idx..idx+token_len].iter().collect::<String>().as_str() {
            "(" => Token::OpenBrace,
            ")" => Token::CloseBrace,
            "+" => Token::Plus,
            "*" => Token::Times,
            s   => Token::Number(s.parse::<i64>().unwrap()),
        });

        idx += token_len;

    }

    results
}

#[derive(Debug, Clone)]
enum Token {
    Number(i64),
    Times,
    Plus,
    OpenBrace,
    CloseBrace
}

fn parse(input: &[Token]) -> ParseItem {
    // The top level has additions or multiplications, or it's inside parens, or it's a
    // single number

    match input.len() {
        0 => panic!("Expected a non-empty input!"),
        1 => {
            if let Token::Number(v) = input[0] {
                ParseItem {
                    entry: GrammarItem::Number(v),
                    children: Vec::new(),
                }
            } else {
                panic!("A single value was received, but it was not a number: {:?}", input[0]);
            }
        }
        _ => {

            // Multiplication has the lowest precedence, so check if the calculation has any at
            // the lowests level and split at them
            let mut mul_indices: Vec<usize> = Vec::new();
            let mut add_indices: Vec<usize> = Vec::new();
            let mut current_paren_level = 0;
            for (idx, v) in input.iter().enumerate() {
                match v {
                    Token::OpenBrace  => current_paren_level += 1,
                    Token::CloseBrace => current_paren_level -= 1,
                    Token::Plus       => if current_paren_level == 0 { add_indices.push(idx); },
                    Token::Times      => if current_paren_level == 0 { mul_indices.push(idx); },
                    Token::Number(_)  => { }
                }
            }
        
            if mul_indices.len() > 0 {
                // Multiplication is the root element
                ParseItem {
                    entry: GrammarItem::Product,
                    children: split_at_indices(&input, &mul_indices).iter().map(|s| parse(s)).collect(),
                }
            } else if add_indices.len() > 0 {
                // Addition is the root element
                ParseItem {
                    entry: GrammarItem::Sum,
                    children: split_at_indices(&input, &add_indices).iter().map(|s| parse(s)).collect(),
                }
            } else {
                // Parentheses are the root element
                ParseItem {
                    entry: GrammarItem::Paren,
                    children: vec![parse(&input[1..input.len()-1])],
                }
            }
        }
    }
}

fn split_at_indices<T: Clone>(arr: &[T], indices: &[usize]) -> Vec<Vec<T>> {
    let mut results = Vec::new();
    let mut next_range_start = 0;
    
    for split in indices {
        results.push(arr[next_range_start..*split].into());
        next_range_start = split + 1;

    }
    results.push(arr[next_range_start..].into());
    results
}


#[derive(Debug)]
enum GrammarItem {
    Product,
    Sum,
    Number(i64),
    Paren
}

#[derive(Debug)]
struct ParseItem {
    entry: GrammarItem,
    children: Vec<ParseItem>,
}

impl ParseItem {
    fn of_number(input: i64) -> ParseItem {
        ParseItem {
            entry: GrammarItem::Number(input),
            children: Vec::new(),
        }
    }

    fn parse_1(input: &[char]) -> ParseItem {
        // Valid input starts with a number or a paren. If the first char is an opening
        // paren, find the matching closing paren, and parse the contents. If the first
        // char is a digit, find the operator after it and generate a ParseItem of the
        // operation.
        let mut idx: usize = input.len() - 1;
        let saw_number = input[idx].is_digit(10);
        while idx > 1 && input[idx-1].is_digit(10) {
            idx -= 1;
        }

        let right_op: ParseItem;

        if saw_number {
            // A number was encountered
            right_op = ParseItem::of_number(
                input[idx..]
                    .iter()
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap());
        } else {
            // A paren was encountered
            assert_eq!(')', input[input.len() - 1]);
            // Find the matching paren
            let mut current_parens: i8 = 1;
            while current_parens > 0 {
                idx -= 1;
                current_parens += match input[idx] {
                    ')' => 1,
                    '(' => -1,
                    _ => 0,
                };
            }
            assert_eq!('(', input[idx]);
            right_op = ParseItem::parse_1(&input[idx+1..input.len()-1]);
        }

        if idx == 0 {
            right_op
        } else {
            idx -= 1;
            match input[idx] {
                '*' => ParseItem {
                    entry: GrammarItem::Product,
                    children: vec![ParseItem::parse_1(&input[..idx]), right_op],
                },
                '+' => ParseItem {
                    entry: GrammarItem::Sum,
                    children: vec![ParseItem::parse_1(&input[..idx]), right_op],
                },
                _ => panic!("Unexpected character: {}", input[idx]),
            }
        }
    }

    fn evaluate(&self) -> i64 {
        match self.entry {
            GrammarItem::Product   => self.children.iter().map(ParseItem::evaluate).product(),
            GrammarItem::Sum       => self.children.iter().map(ParseItem::evaluate).sum(),
            GrammarItem::Number(o) => o,
            GrammarItem::Paren     => self.children[0].evaluate(),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_day18_example_task_1() {
        assert_eq!(71, crate::task_1_internal(&"1 + 2 * 3 + 4 * 5 + 6".to_string()));
        assert_eq!(51, crate::task_1_internal(&"1 + (2 * 3) + (4 * (5 + 6))".to_string()));
        assert_eq!(26, crate::task_1_internal(&"2 * 3 + (4 * 5)".to_string()));
        assert_eq!(437, crate::task_1_internal(&"5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()));
        assert_eq!(
            12240,
            crate::task_1_internal(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string())
        );
        assert_eq!(
            13632,
            crate::task_1_internal(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string())
        );
    }

    #[test]
    fn verify_day18_example_task_2() {
        assert_eq!(231, crate::task_2_internal(&"1 + 2 * 3 + 4 * 5 + 6".to_string()));
        assert_eq!(51, crate::task_2_internal(&"1 + (2 * 3) + (4 * (5 + 6))".to_string()));
        assert_eq!(46, crate::task_2_internal(&"2 * 3 + (4 * 5)".to_string()));
        assert_eq!(1445, crate::task_2_internal(&"5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()));
        assert_eq!(
            669060,
            crate::task_2_internal(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string())
        );
        assert_eq!(
            23340,
            crate::task_2_internal(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string())
        );
    }
}
