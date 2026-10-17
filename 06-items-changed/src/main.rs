use adw::prelude::*;
use gtk::glib;

mod filters;
mod sorters;
mod task;
mod task_row;
mod task_store;
mod window;

const APP_ID: &str = "dev.fromthearchitect.gtkwidgets.ItemsChanged";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(window::build);
    app.run()
}
