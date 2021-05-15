use rolodex::rolodex;

fn main() {
    println!("Hello, world!");
    let mut dex = rolodex::Contact::new(String::from("Ben"), String::from("Man"));
    dex.add_entry(String::from("Some data"));
    println!("{:?}", dex);
}
