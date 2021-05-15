use rolodex::rolodex;
use ::rolodex::files;

fn main() {
    read_test();
}

fn read_test() {
    let dex = files::load_rolodex("test.txt");
    println!("{:?}", dex);
}

fn write_test() {
    let mut contact = rolodex::Contact::new(String::from("Ben"), String::from("Man"));
    contact.add_entry(String::from("Some data"));
    let mut dex = rolodex::Rolodex::new();
    dex.add_contact(contact);
    println!("{:?}", dex);

    files::save_rolodex(dex, "test.txt");
}
