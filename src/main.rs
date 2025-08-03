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
fn contanis_math_op(vec:Vec<String>)->bool{
    let ops=["+","-","*","/"];
    vec.iter().any(|item| ops.contains(&item.as_str()))
}
fn is_math_op(s: &str) -> bool {
    matches!(s, "+" | "-" | "*" | "/")
}

fn tokenize_expression(input: &str) -> Vec<String> {
    let spaced = input
        .replace('\n', " __NEWLINE__ ")
        .replace('+', " + ")
        .replace('-', " - ")
        .replace('*', " * ")
        .replace('/', " / ");

    spaced
        .split_whitespace()
        .map(|s| {
            if s == "__NEWLINE__" {
                "\n".to_string()
            } else {
                s.to_string()
            }
        })
        .collect()
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

     let randx: Vec<&str> = vec!["20", "+", "30"];
    let testx_string = randx.join(""); // "20+30"

    println!("The string form of that array is: {}", testx_string);

    let res=meval::eval_str(testx_string).unwrap();
    println!("meavl res is {}",res)
  

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
            let words = tokenize_expression(inner);

            println!("Extracted inner expression: {:?}", words);

            if contanis_math_op(words.clone()) {
                process_math_expression(words, &mut parts);
            } else {
                // Handle single variable or text expressions
                let mut combined = String::new();
                for word in words {
                    if combined.is_empty() {
                        combined = search_for_vars(&word.as_str());
                    } else {
                        combined = combined + " " + &search_for_vars(&word.as_str());
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

    // If any remaining text after last }}
    if !text.is_empty() {
        parts.push(text);
    }

    parts.join("")
}

fn process_math_expression(words: Vec<String>, parts: &mut Vec<String>) {
    let mut meval_vec: Vec<String> = Vec::new();
    let mut have_string = false;
    let mut have_number = false;

    for word in words {
        if word == "\n" {
            // Finalize current expression on newline
            finalize_expression(&meval_vec, have_string, have_number, parts);
            // Reset state for next expression
            meval_vec.clear();
            have_string = false;
            have_number = false;
            continue;
        }

        if is_math_op(&word.as_str()) {
            meval_vec.push(word.to_string());
            continue;
        }

        match resolve_operand(&word.as_str()) {
            OperandResolution::Number(n) => {
                if have_string {
                    parts.push("we cant add string to number ".to_string());
                    // Reset state
                    meval_vec.clear();
                    have_string = false;
                    have_number = false;
                    continue;
                }
                have_number = true;
                meval_vec.push(n.to_string());
            }
            OperandResolution::StringVar(var_st) => {
                have_string = true;
                if have_number {
                    parts.push("we cant add string to number ".to_string());
                    // Reset state
                    meval_vec.clear();
                    have_string = false;
                    have_number = false;
                    continue;
                }
                meval_vec.push(var_st.to_string());
            }
            OperandResolution::NotFound => {
                parts.push("VAR not found check the spling ".to_string());
                meval_vec.clear();
                have_string = false;
                have_number = false;
                continue;
            }
            OperandResolution::Invalid => {
                parts.push("Invalid ".to_string());
                meval_vec.clear();
                have_string = false;
                have_number = false;
                continue;
            }
        }
    }

    // Finalize any remaining expression after loop
    if !meval_vec.is_empty() {
        finalize_expression(&meval_vec, have_string, have_number, parts);
    }
}

fn finalize_expression(meval_vec: &[String], have_string: bool, have_number: bool, parts: &mut Vec<String>) {
    if have_string && !have_number {
        parts.push(meval_vec.join(""));
    } else if !have_string && have_number {
        match meval::eval_str(meval_vec.join("")) {
            Ok(r) => parts.push(r.to_string()),
            Err(_) => println!("error"),
        }
    }
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