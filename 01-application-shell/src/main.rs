use adw::prelude::*;
use gtk::glib;

mod window;

const APP_ID: &str = "dev.fromthearchitect.gtkwidgets.ApplicationShell";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(window::build);
    app.run()
}
