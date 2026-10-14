use adw::prelude::*;
use gtk::glib;

mod multi;
mod single;
mod task;
mod task_row;
mod window;

const APP_ID: &str = "dev.fromthearchitect.gtkwidgets.Selection";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(window::build);
    app.run()
}
