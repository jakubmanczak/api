use sqlite;

fn main() {
    let connection = sqlite::open("manczak.db").unwrap();
    let query = "SELECT * FROM splashes;";
    
    connection.execute(query).unwrap();
    connection.iterate(query, |pairs| {
        for &(name, value) in pairs.iter() {
            println!("{} => {}", name, value.unwrap());
        }
        println!("----");
        return true;
    }).unwrap();
}
