

//this form is not possible without initializing the class at top level and then passing it around to all of the child functions. It would lead to only one file, but it would make the logic harder to follow in the initialization stage without some form of redux which would add further complication and is unneccessary. Multiple files are easier to understand, until the style! problem is resolved.


// use std::collections::HashMap;

// struct Styles{
//   canvas: HashMap<String, String>, 
//   home: HashMap<String, String>, 
//   main: HashMap<String, String>,
//   mdpages: HashMap<String, String>,
//   paintbar: HashMap<String, String>,
// }

// impl Styles{
//   fn carriage_return(&self) -> Self {
//     let new_contents = self.contents.replace("\n", "<br/>");
//     let p = Parse{contents: new_contents};
//     p
//   }
//   pub fn canvas(&self) -> Self{
//     let mut newstyles = self.canvas
//     style.insert("main".to_string(), "
//         display: flex;
//         flex-direction: row;
//         height: 100vh;
//         background: lightgrey;
//     ".to_string());
//     style.insert("flexcontainer".to_string(),"
//         display: flex;
//         width: 100%;
//         flex-direction: row;
//     ".to_string());
//     style.insert("flex1".to_string(),"
//         flex: 1;
//     ".to_string());
//     style.insert("paintbarcontainer".to_string(),"
//         flex: 1;
//         background: darkgrey;
//     ".to_string());
//     style.insert("nav_item".to_string(), "font-size: 20px;".to_string());

//     style
// }


// }