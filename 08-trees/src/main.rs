use adw::prelude::*;
use gtk::glib;

mod columns;
mod library;
mod node;
mod outline;
mod window;

const APP_ID: &str = "dev.fromthearchitect.gtkwidgets.Trees";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(window::build);
    app.run()
}
