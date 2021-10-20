use std::process::Command;

pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
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
            .arg(&output)
            .arg("--jsonArray")
            .status()
            .expect("mongo list collection command failed to start");
        
        println!("{}", &exportcmd);
    }



}

// fn import() {

// }