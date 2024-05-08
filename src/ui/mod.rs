use super::notd;
use gtk::{
    gdk::Display,
    gio::{ActionEntry, Menu, MenuItem},
    prelude::*,
    AboutDialog, Align, Application, ApplicationWindow, Box, Entry, EntryBuffer, HeaderBar, Label, MenuButton, Orientation,
};

pub fn build_ui(app: &Application) {
    let number_label = Label::builder()
        .label(format!(
            "<span size=\"600%\">{}</span>",
            notd::number_of_the_day().to_string()
        ))
        .use_markup(true)
        .build();

    let name_buffer = EntryBuffer::new(Some("Name"));

    let name_entry = Entry::with_buffer(&name_buffer);

    let subtitle_label = Label::builder()
        .label("<span font_weight=\"bold\">Today's Daily Number is...</span>")
        .use_markup(true)
        .build();

    let gtk_box = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();

    gtk_box.append(&name_entry);
    gtk_box.append(&subtitle_label);
    gtk_box.append(&number_label);

    name_entry.connect_changed(move |_| {
        let name = name_buffer.text();
        let pnum = notd::personal_number_of_the_day(&name);
        number_label.set_label(&format!("<span size=\"600%\">{}</span>", pnum));
        if name.trim() == "" {
            subtitle_label
                .set_label("<span font_weight=\"bold\">Today's Daily Number is...</span>");
        } else {
            subtitle_label.set_label("<span font_weight=\"bold\">Your Daily Number is...</span>")
        }
    });

    let titlebar = HeaderBar::builder().build();

    titlebar.set_opacity(1.0);

    let about_menu = MenuItem::new(Some("About"), Some("win.about"));

    let menu = Menu::new();

    menu.append_item(&about_menu);

    let menu_button = MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .primary(true)
        .tooltip_text("Menu")
        .menu_model(&menu)
        .build();

    titlebar.pack_end(&menu_button);

    let action_about = ActionEntry::builder("about")
        .activate(|window: &ApplicationWindow, _, _| {
            show_about(&window);
        })
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Number of the Day")
        .child(&gtk_box)
        .titlebar(&titlebar)
        .width_request(300)
        .height_request(400)
        .build();

    window.add_action_entries([action_about]);

    window.present();
}

fn show_about(window: &ApplicationWindow) {
    let about = AboutDialog::builder()
        .transient_for(window)
        .program_name("Number of the Day")
        .authors(["Venomade"])
        .version("0.0.2")
        .copyright("© 2024 Venomade")
        .license_type(gtk::License::Gpl30)
        .comments("Shows a number of the day, general or personal")
        .website("https://github.com/venomade/numberoftheday")
        .build();

    about.present();
}

pub fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_string(include_str!("ui.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not get display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
