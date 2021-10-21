use std::process::Command;
use std::fs;

pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn export(database: &str, query: &str, output: &str) -> () {
    println!("exporting {} using query {} to location {}", database, query, output);
    
    let collections = Command::new("mongo")
        .arg(&database)
        .arg("--quiet")
        .arg("--eval")
        .arg("db.getCollectionNames()")
        .output()
        .expect("mongo list collection command failed to start");
    
    let mut stdout: String = String::from_utf8_lossy(&collections.stdout).to_string().to_owned();

    stdout =  remove_whitespace(&*stdout);    

    stdout = stdout.replace(&['[', ']', '\"'][..], ""); 

    let collectionlist = stdout.split(',');

    fs::create_dir_all(format!("./{}", &output))
        .expect("failed to create directory");

    for collection in collectionlist {
        println!("Going through collection: {}", &collection);
        let filename = format!("./{}/{}.json", &output, &collection);
        println!("{}", &filename);
        
        let exportcmd = Command::new("mongoexport")
            .arg("-d")
            .arg(&database)
            .arg("-c")
            .arg(&collection)
            .arg("--query")
            .arg(&query)
            .arg("--out")
            .arg(&filename)
            .arg("--jsonArray")
            .status()
            .expect("mongo export collection command failed to start");
        
        println!("{}", &exportcmd);
    }



}


pub fn import(database: &str, input: &str ) -> () {
    let mut filepath = format!("./{}",&input); 
    let fileexists = path_exists(&filepath);
    if fileexists {
        let paths = fs::read_dir(filepath).unwrap();
        for path in paths {
            filepath = path.unwrap().path().display().to_string();
            if filepath.contains(".json"){
                println!("Processing: {}", filepath);
                let filedir_split = filepath.split("/");
                let mut collection: &str = ""; 
                for part in filedir_split {
                    if part.contains(".json") {
                        println!("fname {}", &part);
                        let subpart = part.split(".");
                        for sub in subpart {
                            if !sub.contains("json") {
                                collection = &sub;
                            }
                        }
                    }
                }
                println!("coll: {}", &collection);
                let importcmd = Command::new("mongoimport")
                    .arg("-d")
                    .arg(&database)
                    .arg("-c")
                    .arg(&collection)
                    .arg("--file")
                    .arg(&filepath)
                    .arg("--jsonArray")
                    .status()
                    .expect("mongo export collection command failed to start");
                
                println!("{}", &importcmd);
            }
            else{
                println!("not a json file");
            }
        }
    }
    else{
        println!("file doesnt exist");
    }
}