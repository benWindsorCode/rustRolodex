use rolodex::rolodex;
use ::rolodex::files;
use ::rolodex::interface;

fn main() {
    interface::start_interface();
    // write_test();
}

fn read_test() {
    let dex = files::load_rolodex("test.txt");
    println!("{:?}", dex);
}

fn write_test() {
    let mut contact_1 = rolodex::Contact::new(String::from("Ben"), String::from("Windsor"));
    contact_1.add_entry(String::from("Some data about meeting with Ben"));

    let mut contact_2 = rolodex::Contact::new(String::from("Charles"), String::from("Samuel"));
    contact_2.add_entry(String::from("Some data about coffee with Charles"));

    let mut contact_3 = rolodex::Contact::new(String::from("Sophia"), String::from("Marple"));
    contact_3.add_entry(String::from("Some notes about meeting with Sophie"));

    let mut dex = rolodex::Rolodex::new();
    dex.add_contact(contact_1);
    dex.add_contact(contact_2);
    dex.add_contact(contact_3);
    println!("{:?}", dex);

    files::save_rolodex(dex, "test.txt");
}
