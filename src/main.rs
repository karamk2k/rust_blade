use std::fs;
use std::collections::HashMap;
use serde_json::Value;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::io::Write;

macro_rules! var_map {
    ($($var:ident),*) => {
        {
            let mut map = GLOBAL_VARS.lock().unwrap();
            $(
                map.vars.insert(stringify!($var).to_string(), serde_json::json!($var));
            )*
        }
    };
}

#[derive(Debug)]
struct VarCont {
    vars: HashMap<String, Value>,
}

enum OperandResolution {
    Number(i64),
    StringVar(String),
    NotFound,
    Invalid,
}

static GLOBAL_VARS: Lazy<Mutex<VarCont>> = Lazy::new(|| {
    Mutex::new(VarCont {
        vars: HashMap::new(),
    })
});

fn get_var(key: &str) -> Option<Value> {
    let vars = GLOBAL_VARS.lock().unwrap();
    vars.vars.get(key).cloned()
}

fn main() {
    let tw = "Idont know ";
    let hello = "the vars from rust to html ";
    let ktk = 30;
    let x: i32 = 10;
    let y: i32 = 20;
    var_map!(hello, ktk, tw, x, y);
    
    let text = fs::read_to_string("test.html").expect("Failed to read file");
    let new_text = get_tokens(text);
    let mut file = fs::File::create("output.html").expect("Failed to create file");
    file.write_all(new_text.as_bytes()).expect("Failed to write to file");
    println!("File written successfully with the new content.");
    
    let vars = GLOBAL_VARS.lock().unwrap();
    println!("the map is {:#?}", vars);
}

fn get_tokens(mut text: String) -> String {
    if text.is_empty() {
        return String::new();
    }

    let mut parts: Vec<String> = Vec::new();

    while let Some(start) = text.find("{{") {
        if let Some(end) = text.find("}}") {
            // Text before {{
            if start > 0 {
                parts.push(text[..start].to_string());
            }

            // Extract inner expression
            let inner = &text[start + 2..end];
            let words: Vec<&str> = inner.split_whitespace().collect();
            println!("Extracted inner expression: {:?}", words);

            // Handle expressions with operators
            if words.len() == 3 && words[1] == "+" {
                let left = resolve_operand(words[0]);
                let right = resolve_operand(words[2]);
                
                match (left, right) {
                    (OperandResolution::Number(a), OperandResolution::Number(b)) => {
                        parts.push((a + b).to_string());
                    }
                    (OperandResolution::StringVar(a), OperandResolution::StringVar(b)) => {
                        parts.push(a + &b);
                    }
                    (OperandResolution::StringVar(a), OperandResolution::Number(b)) => {
                        parts.push(a + &b.to_string());
                    }
                    (OperandResolution::Number(a), OperandResolution::StringVar(b)) => {
                        parts.push(a.to_string() + &b);
                    }
                    (OperandResolution::NotFound, _) | (_, OperandResolution::NotFound) => {
                        parts.push("#MISSING_VAR#".to_string());
                    }
                    _ => {
                        parts.push("#INVALID_OPERANDS#".to_string());
                    }
                }
            } else {
                // Handle single variable lookups
                let mut combined = String::new();
                for word in words {
                    if combined.is_empty() {
                        combined = search_for_vars(word);
                    } else {
                        combined = combined + " " + &search_for_vars(word);
                    }
                }
                parts.push(combined);
            }

            // Update remaining text
            text = text[end + 2..].to_string();
        } else {
            break;
        }
    }

    // Push leftover text
    if !text.is_empty() {
        parts.push(text);
    }

    parts.join("")
}

fn search_for_vars(name: &str) -> String {
    match get_var(name) {
        Some(val) => {
            if let Some(s) = val.as_str() {
                s.to_string()
            } else if let Some(n) = val.as_i64() {
                n.to_string()
            } else {
                name.to_string()
            }
        }
        None => name.to_string(),
    }
}

fn resolve_operand(word: &str) -> OperandResolution {
    // Try to parse as number directly
    if let Ok(num) = word.parse::<i64>() {
        return OperandResolution::Number(num);
    }

    // Check if it's a variable
    if let Some(val) = get_var(word) {
        if let Some(num) = val.as_i64() {
            OperandResolution::Number(num)
        } else if let Some(s) = val.as_str() {
            OperandResolution::StringVar(s.to_string())
        } else {
            OperandResolution::Invalid
        }
    } else {
        OperandResolution::NotFound
    }
}