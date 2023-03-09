use std::collections::HashMap;

pub fn styles() -> HashMap<String, String>{
    let mut style = HashMap::new();
    style.insert("main".to_string(), "background: rgba(0,0,0,0.2);".to_string());
    style.insert("nav_item".to_string(), "font-size: 20px;".to_string());
    style.insert("colorwheelcontainer".to_string(), "
        background: lightblue;
        margin: 10px;
        min-height: 20vw;
        width: calc( 100% - 20px );
    ".to_string());
    style
}
