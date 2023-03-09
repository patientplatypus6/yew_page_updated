use std::collections::HashMap;

pub fn styles() -> HashMap<String, String>{
    let mut style = HashMap::new();
    style.insert("main".to_string(), "
        background: rgba(0,0,0,0.4);
    ".to_string());
    style.insert("newspapertitle".to_string(), "
        color: rgba(0,0,100,0.8);
        font-family: 'Righteous', sans-serif;
    ".to_string());
    style.insert("nav_item".to_string(), "font-size: 20px;".to_string());
    style.insert("newspapertitle".to_string(), "
        font-size: 20px;
        font-weight: bold;
        text-align: center;
    ".to_string());
    style
}
