
use std::fs;
use std::collections::HashMap;
use serde_json::Value;
use once_cell::sync::Lazy;
use std::sync::Mutex;
macro_rules! var_to_string {
    ($var:ident) => {
        format!("{} = {:?}", stringify!($var), $var)
    };
}

macro_rules! var_map {
    ( $( $var:ident ),* ) => {{
        let mut map = GLOBAL_VARS.lock().unwrap();
        $(
            map.vars.insert(stringify!($var).to_string(), serde_json::json!($var));
        )*
    }};
}

#[derive(Debug)]
struct VarCont {
   vars:HashMap<String,Value>, 
}


static GLOBAL_VARS: Lazy<Mutex<VarCont>>= Lazy::new(||{
    Mutex::new(VarCont{
        vars:HashMap::new(),
    })
});
fn get_var(key: &str) -> Option<Value> {
    let vars = GLOBAL_VARS.lock().unwrap();
    println!("the map is {:#?}",vars);
    vars.vars.get(key).cloned()
}

fn main() {
let tw="Idont know ";    
 let hello="the vars from rust to html ";
 let ktk=30;
 var_map!(hello);
 var_map!(ktk);
 var_map!(tw);
    let text = fs::read_to_string("test.html").expect("Failed to read file");
    let new_text = get_tokens(text);
   
    
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

            // Text inside {{...}}
             let mut inner = &text[start + 2..end];
             let words:Vec<&str>=inner.split_whitespace().collect();
             //println!("the new vec : {:?}",words);
             for word in words{
             let new_inner=search_for_vars(word);
            parts.push(new_inner);

            }
             
            println!("the iner is {}",inner);
            // Update remaining text
            text = text[end + 2..].to_string();
        } else {
            break;
        }
    }

    // Any remaining part after last }}
    if !text.is_empty() {
        parts.push(text);
    }

    // Debug print
    println!("{:#?}", parts);

    // Rejoin as single string or return parts as needed
    parts.join("")
}

fn search_for_vars(name: &str) -> String {
    match get_var(name) {
        Some(val) => {
            println!("found it {}",name);
            val.as_str().unwrap_or(name).to_string()
        },
        None => {
            println!("not found {} ",name);
            name.to_string()
        }
    }
}
