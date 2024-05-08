mod notd;
mod ui;

use gtk::Application;
use gtk::glib;
use gtk::prelude::*;
use ui::{build_ui, load_css};

const APP_ID: &str = "com.venomade.notd";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run()
}
