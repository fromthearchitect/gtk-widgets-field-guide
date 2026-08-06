use adw::prelude::*;
use gtk::glib;

const APP_ID: &str = "dev.fromthearchitect.gtkwidgets.ApplicationShell";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &adw::Application) {
    let header = adw::HeaderBar::new();

    let content = gtk::Label::builder()
        .label("The shell is the chrome around this label.")
        .build();

    let toolbar_view = adw::ToolbarView::new();
    toolbar_view.add_top_bar(&header);
    toolbar_view.set_content(Some(&content));

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Application Shell")
        .default_width(420)
        .default_height(320)
        .content(&toolbar_view)
        .build();

    window.present();
}
