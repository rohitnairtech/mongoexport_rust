#[macro_use]
extern crate clap;
use clap::App;

mod helpers;
use helpers::{export, import, remove_whitespace};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    
    let database = remove_whitespace(matches.value_of("database").unwrap());
    let operation = remove_whitespace(matches.value_of("operation").unwrap());

    if operation == "export" {
        let query = remove_whitespace(matches.value_of("query").unwrap());
        let output = remove_whitespace(matches.value_of("output").unwrap_or("output"));
        export(&database, &query, &output);
    }
    else{
        println!("import fn hai");
        let input = remove_whitespace(matches.value_of("input").unwrap_or("output"));
        import(&database, &input);
    }

}


