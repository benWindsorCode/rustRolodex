use cursive::views::{Dialog, Button, TextView, EditView, LinearLayout, SelectView, DummyView};
use cursive::{event::Key, menu, Cursive};
use cursive::traits::*;
use crate::files;
use crate::rolodex::Rolodex;

pub fn start_interface() {
    let mut siv = cursive::default();

    siv.menubar()
        .add_subtree(
            "File",
            menu::MenuTree::new()
                .leaf("New Rolodex", |s| {
                    s.set_user_data(Rolodex::new());
                    let rolodex = Rolodex::new();
                    s.add_layer(show_rolodex(&rolodex));
                    s.set_user_data(rolodex);
                })
                .leaf("Load Rolodex", |s| {
                    s.add_layer(load_rolodex_dialog())
                })
                .leaf("Save Rolodex", |s| {
                    s.add_layer(Dialog::info("Saving"))
                })
                // .leaf("Show Rolodex", |s| {
                //     let rolodex = s.take_user_data::<Rolodex>().unwrap();
                //     s.add_layer(show_rolodex(&rolodex));
                //     s.set_user_data(rolodex);
                // })
        )
        .add_leaf(
            "Quit",
            |s| s.quit()
        );

    siv.set_autohide_menu(false);

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    siv.run();
}

fn load_rolodex_dialog() -> Dialog {
    let dialog = Dialog::new()
        .title("New Transaction")
        .content(
            LinearLayout::vertical()
            .child(TextView::new("File Path:"))
            .child(EditView::new().on_submit(load_rolodex).with_name("load_file_path"))
        )
        .button(
            "LOAD",
            |s| {
                let path = s.call_on_name("load_file_path", |view: &mut EditView| view.get_content()).unwrap();
                
                load_rolodex(s, &path);
            }
        )
        .button(
            "CANCEL",
            |s| { s.pop_layer(); }
        );
    
    dialog
}

fn load_rolodex(s: &mut Cursive, path: &str) {
    let rolodex = files::load_rolodex(path);

    // On sucesfull load, close dialog and persist, otherwise allow user to retry
    match rolodex {
        Ok(rolodex) => {
            s.pop_layer();
            s.add_layer(show_rolodex(&rolodex));
            s.set_user_data(rolodex);
        }
        Err(e) => s.add_layer(Dialog::info(format!("Failed to load rolodex: {}", e)))
    }
}

fn show_rolodex(rolodex: &Rolodex) -> Dialog {

    let mut select_contact = SelectView::<String>::new()
        .on_submit(show_entry);

    for contact in &rolodex.contacts {
        select_contact.add_item_str(format!("{} {}", &contact.first_name, &contact.last_name));
    }

    // Add name after as the named view has no add_item_str on it
    let select_contact = select_contact.with_name("select_contact");

    let show_contact = TextView::new("No Contact Selected").with_name("show_contact_text");

    let select_dialog = LinearLayout::horizontal()
        .child(select_contact)
        .child(DummyView)
        .child(show_contact);

    let buttons = LinearLayout::horizontal()
        .child(Button::new("New Contact", |s| {} ))
        .child(Button::new("Add Entry", |s| {} ))
        .child(Button::new("Edit Entry", |s| {} ))
        .child(Button::new("Close", |s| { s.pop_layer(); }));

    let dialog = Dialog::around(LinearLayout::vertical()
        .child(select_dialog)
        .child(DummyView)
        .child(buttons)
    );

    dialog
}

fn show_entry(s: &mut Cursive, contact: &str) {
    s.call_on_name("show_contact_text", |view: &mut TextView| {
        view.set_content(format!("Showing entries for: {}", contact))
    });
}