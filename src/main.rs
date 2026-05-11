use std::collections::HashMap;
use std::iter::Peekable;
use std::num::ParseFloatError;
use std::str::Chars;
use std::fs;

#[derive(Debug)]
#[allow(dead_code)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

#[derive(Debug)]
enum JsonValueType {
    Null,
    Bool,
    Number,
    String,
    Array,
    Object,
}

fn advance_chars(chars: &mut Peekable<Chars>, amount: usize) -> usize {
    for _i in 0..amount {
        chars.next();
    }
    amount
}

fn parse_string(input: &String, starting_index: usize) -> Result<(String, usize), String> {
    let mut chars = input.chars().peekable();
    let mut result = String::new();
    let mut i = advance_chars(&mut  chars, starting_index);

    while let Some(&c) = chars.peek() {
        if c == '"' {
            break;
        }
        result.push(c);
        chars.next();
        i += 1;
    }

    Ok((result, i))
}

fn determinate_value_type(input: &String, starting_index: usize) -> Option<(JsonValueType, usize)> {
    let mut chars = input.chars().peekable();
    let mut i = advance_chars(&mut  chars, starting_index);

    while let Some(&c) = chars.peek() {
        i += 1;
        match c {
            '"' => {
                return Some((JsonValueType::String, i))
            },
            '-' | '0'..='9' => {
                return Some((JsonValueType::Number, i))
            }
            '{' => {
                return Some((JsonValueType::Object, i))
            }
            '[' => {
                return Some((JsonValueType::Array, i))
            }
            ']' | '}' => {
                break
            }
            'n' => {
                let mut expected = String::new();
                expected.push('n');
                for _j in 0..3 {
                    chars.next();
                    expected.push(*chars.peek().unwrap());
                }
                if expected.eq("null") {
                    return Some((JsonValueType::Null, i));
                }
            }
            't' => {
                let mut expected = String::new();
                expected.push('t');
                for _j in 0..3 {
                    chars.next();
                    expected.push(*chars.peek().unwrap());
                }
                if expected.eq("true") {
                    return Some((JsonValueType::Bool, i));
                }
            }
            'f' => {
                let mut expected = String::new();
                expected.push('f');
                for _j in 0..4 {
                    chars.next();
                    expected.push(*chars.peek().unwrap());
                }
                if expected.eq("false") {
                    return Some((JsonValueType::Bool, i));
                }
            }
            _ => {
                chars.next();
            }
        }

    }

    None
}

fn parse_bool(input: &String, starting_index: usize) -> Result<(bool, usize), String> {
    let mut chars = input.chars().peekable();
    let i = advance_chars(&mut  chars, starting_index);

    let first = *chars.peek().unwrap();
    match first {
        't' => {
            let mut expected = String::new();
            expected.push('t');
            for _j in 0..3 {
                chars.next();
                expected.push(*chars.peek().unwrap());
            }
            if expected.eq("true") {
                Ok((true, i + 3))
            } else {
                Err(format!("Unable to parse {} as bool", expected))
            }
        }
        'f' => {
            let mut expected = String::new();
            expected.push('f');
            for _j in 0..4 {
                chars.next();
                expected.push(*chars.peek().unwrap());
            }
            if expected.eq("false") {
                Ok((false, i + 4))
            } else {
                Err(format!("Unable to parse {} as bool", expected))
            }
        },
        _ => todo!()
    }
}

fn parse_number(input: &String, starting_index: usize) -> Result<(f64, usize), ParseFloatError> {
    let mut chars = input.chars().peekable();
    let mut result = String::new();
    let mut i = advance_chars(&mut  chars, starting_index);

    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() || c == '.' || c == '-' {
            result.push(c);
        } else if !result.is_empty() {
            i -= 1;
            break
        }
        chars.next();
        i += 1;
    }

    let number = result.parse::<f64>();
    if let Err(e) = number {
        return Err(e);
    }

    Ok((number?, i))
}

fn parse_object(input: &String, starting_index: usize) -> Result<(HashMap<String, JsonValue>, usize), String> {
    let bytes = input.as_bytes();
    let mut i = starting_index;

    let mut map: Option<HashMap<String, JsonValue>> = None;
    let mut current_key: Option<String> = None;

    while i < bytes.len() {
        let char = bytes[i] as char;

        match char {
            '{' => {
                if map.is_none() {
                    map = Some(HashMap::new());
                } else {
                    // todo
                }
            }
            '}' => {
                break
            }
            '"' => {
                if map.is_none() {
                    return Err("Root object required".to_string());
                }

                if current_key.is_some() {
                    return Err(format!("Missing comma after '{}' key", current_key.unwrap()))
                }

                let key_result = parse_string(input, i + 1);
                if let Err(e) = key_result {
                    return Err(e);
                }
                let tuple = key_result.unwrap();


                current_key = Some(tuple.0);
                println!("Key: {:?}", current_key);
                i = tuple.1 + 1;
                continue
            }
            ':' => {
                if map.is_none() {
                    return Err("Root object required".to_string());
                }

                if current_key.is_none() {
                    return Err("Missing key before colon".to_string());
                }

                let value_type_tuple = determinate_value_type(input, i + 1);
                if value_type_tuple.is_none() {
                    break
                }
                let value_type_tuple = value_type_tuple.unwrap();
                let value_type = value_type_tuple.0;
                let value_start_index = value_type_tuple.1;

                match value_type {
                    JsonValueType::Number => {
                        if let Some(ref mut m) = map {
                            let number = parse_number(input, value_start_index - 1);
                            if let Ok(n) = number {
                                m.insert(current_key.clone().unwrap(), JsonValue::Number(n.0));
                                i = n.1 + 1;
                                continue
                            } else if let Err(e) = number {
                                return Err(e.to_string());
                            }

                        }
                    }
                    JsonValueType::String => {
                        if let Some(ref mut m) = map {
                            let string = parse_string(input, value_start_index);
                            if let Ok(str) = string {
                                m.insert(current_key.clone().unwrap(), JsonValue::String(str.0));
                                i = str.1 + 1;
                                continue
                            } else if let Err(e) = string {
                                return Err(e);
                            }
                        }
                    }
                    JsonValueType::Object => {
                        if let Some(ref mut m) = map {
                            let object = parse_object(input, value_start_index - 1);
                            if let Ok(obj) = object {
                                m.insert(current_key.clone().unwrap(), JsonValue::Object(obj.0));
                                i = obj.1 + 1;
                                continue
                            } else if let Err(e) = object {
                                return Err(e);
                            }
                        }
                    }
                    JsonValueType::Array => {
                        if let Some(ref mut m) = map {
                            let array = parse_array(input, value_start_index - 1);
                            if let Ok(ar) = array {
                                m.insert(current_key.clone().unwrap(), JsonValue::Array(ar.0));
                                i = ar.1 + 1;
                                continue
                            } else if let Err(e) = array {
                                return Err(e);
                            }
                        }
                    }
                    JsonValueType::Null => {
                        if let Some(ref mut m) = map {
                            m.insert(current_key.clone().unwrap(), JsonValue::Null);
                            i += 5;
                        }
                    }
                    JsonValueType::Bool => {
                        if let Some(ref mut m) = map {
                            let bool = parse_bool(input, value_start_index - 1);
                            if let Ok(b) = bool {
                                m.insert(current_key.clone().unwrap(), JsonValue::Bool(b.0));
                                i = b.1 + 1;
                                continue
                            } else if let Err(e) = bool {
                                return Err(e);
                            }
                        }
                    }
                }
            }
            ',' => {
                current_key = None;
            }
            '\n' | '\t' | ' ' => {

            }
            _ => {
                return Err(format!("Unexpected character '{}' at {}", char.escape_default(), i))
            }
        }
        i += 1;
    }

    Ok((map.unwrap(), i))
}

fn parse_array(input: &String, starting_index: usize) -> Result<(Vec<JsonValue>, usize), String> {
    let bytes = input.as_bytes();
    let mut i = starting_index + 1;

    let mut array: Vec<JsonValue> = Vec::new();

    while i < bytes.len() {
        let char = bytes[i] as char;

        if char.is_whitespace() {
            i += 1;
            continue
        }

        if char == ',' {
            i += 1;
            continue
        }

        let element_type = determinate_value_type(input, i);
        if element_type.is_none() { break; }
        let element_type = element_type.unwrap();
        let element_starting_index = element_type.1;
        let element_type = element_type.0;

        match element_type {
            JsonValueType::Number => {
                let number = parse_number(input, element_starting_index - 1);
                if let Err(e) = number {
                    return Err(e.to_string());
                }
                let number = number.unwrap();
                let new_index = number.1;
                let number = number.0;

                array.push(JsonValue::Number(number));
                i = new_index + 1;
                continue;
            },
            JsonValueType::String => {
                let string = parse_string(input, element_starting_index);
                if let Err(e) = string {
                    return Err(e.to_string());
                }
                let string = string?;
                let new_index = string.1;
                let string = string.0;

                array.push(JsonValue::String(string));
                i = new_index + 1;
                continue
            }
            JsonValueType::Object => {
                let object = parse_object(input, element_starting_index - 1);
                if let Err(e) = object {
                    return Err(e);
                }

                let object = object?;
                let new_index = object.1;
                let map = object.0;

                array.push(JsonValue::Object(map));
                i = new_index + 1;
                continue
            }
            JsonValueType::Array => {
                let child = parse_array(input, element_starting_index - 1);
                if let Err(e) = child {
                    return Err(e);
                }
                let child = child?;
                let new_index = child.1;
                let child = child.0;

                array.push(JsonValue::Array(child));

                i = new_index + 1;
                continue
            }
            JsonValueType::Null => {
                array.push(JsonValue::Null);
                i += 5;
            }
            JsonValueType::Bool => {
                let bool = parse_bool(input, element_starting_index - 1);
                if let Err(e) = bool {
                    return Err(e);
                }

                let bool = bool?;
                let new_index = bool.1;
                let bool = bool.0;

                array.push(JsonValue::Bool(bool));

                i = new_index + 1;
                continue
            }
        }

        i += 1;

    }

    Ok((array, i))
}

fn parse(path: String) -> Result<JsonValue, String> {
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let object = parse_object(&contents, 0);
    if let Err(e) = object {
        return Err(e);
    }

    let tuple = object?;
    let map= tuple.0;

    return Ok(JsonValue::Object(map));
}

fn main() {
    let result = parse(String::from("./example.json"));
    if let Err(e) = result {
        println!("Error: {}", e);
    } else {
        print!("Parsed: {:?}", result.unwrap());
    }
}
