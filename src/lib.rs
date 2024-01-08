use std::collections::HashMap;
//splits
const CONSTANT_SPLITS: [&str; 22] = ["?", "!", ".", ",", ":", ";", "(", ")", "[", "]", "*", "&", "^", "%", "$", "#", "@", "/", "~", "+", "-", "="];
const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "0123456789";

pub fn get_tokens(prompt: &str) -> HashMap<&str, Vec<String>> {
    let tokens: Vec<&str> = prompt.split_whitespace().collect();
    let mut modified_tokens: HashMap<&str, Vec<String>> = HashMap::new();

    // all token types
    let mut str_tokens: Vec<String> = Vec::new();
    let mut num_tokens: Vec<String> = Vec::new();
    let mut distinct_splits: Vec<String> = Vec::new();

    for token in &tokens {
        let mut modified_token = token.to_string();

        for split in CONSTANT_SPLITS.iter() {
            if modified_token.contains(split) {
                distinct_splits.push(split.to_string());
                modified_token = modified_token.replace(split, "");
            }
        }

        if LETTERS.chars().any(|x| modified_token.to_lowercase().contains(x)) {
            str_tokens.push(modified_token.to_lowercase());
        } else if NUMBERS.chars().any(|x| modified_token.to_lowercase().contains(x)) {
            num_tokens.push(modified_token.to_lowercase());
        }
    }

    modified_tokens.insert("str", str_tokens);
    modified_tokens.insert("num", num_tokens);
    modified_tokens.insert("splits", distinct_splits);

    return modified_tokens;
}

pub fn check_for_keywords(tokens: &Vec<String>, keywords: &Vec<&str>) -> (usize, bool) {
    let mut findings = 0;
    for token in tokens {
        for keyword in keywords {
            if token.contains(keyword) {
                findings += 1;
            }
        }
    }
    return (findings, findings >= keywords.len());
}

pub fn get_response_from_vec(tokens: &Vec<String>, lines: &Vec<&str>, def_response: &str) -> String {
    let mut _keywords = Vec::new();
    let mut response = def_response;
    let mut findings = 0;

    for line in lines {
        if line.starts_with(".") {
            continue;
        } else {
            let line_data: Vec<&str> = line.split(":").collect();
            _keywords = line_data[0].split(",").collect();
            response = line_data[1];
            findings = check_for_keywords(&tokens, &_keywords).0;

            if findings >= _keywords.len() {
                break;
            }
        }
    }

    if findings < _keywords.len() {
        response = def_response;
    }

    return response.to_string();
}