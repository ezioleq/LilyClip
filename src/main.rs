#![windows_subsystem = "windows"]

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

mod strings;

fn add_header_button_popover(button: &gtk::MenuButton) {
    let popover = gtk::Popover::new(Some(button));
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 4);

    // Stats button
    let stats = gtk::ModelButton::builder().text("Stats").build();

    // About button
    let about = gtk::ModelButton::builder().text("About").build();
    // Show about window
    about.connect_clicked(|_| {
        let about = gtk::AboutDialog::builder()
            .title(strings::TITLE)
            .version(strings::VERSION)
            .license_type(gtk::License::Gpl30)
            .authors(vec![strings::AUTHOR.into()])
            .build();

        about.show_all();
    });

    // Exit button
    let close = gtk::ModelButton::builder().text("Close").build();
    close.connect_clicked(|_| {
        std::process::exit(0);
    });

    vbox.add(&stats);
    vbox.add(&about);
    vbox.add(&close);
    vbox.show_all();

    popover.add(&vbox);
    popover.set_position(gtk::PositionType::Bottom);

    button.set_popover(Some(&popover));
}

fn build_ui(app: &Application) {
    let win = ApplicationWindow::builder()
        .application(app)
        .default_width(350)
        .default_height(400)
        .title(strings::TITLE)
        .build();

    // Header bar button for misc options
    let csd_button = gtk::MenuButton::new();
    let csd_button_img = gtk::Image::new();
    csd_button_img.set_from_icon_name("open-menu-symbolic".into(), gtk::IconSize::Button);
    csd_button.set_image(Some(&csd_button_img));

    // Popover for header bar button
    add_header_button_popover(&csd_button);

    // CSD header bar
    let header = gtk::HeaderBar::builder()
        .title(strings::TITLE)
        .subtitle(strings::SUBTITLE)
        .has_subtitle(false)
        .show_close_button(true)
        .build();

    // Add and set the new titlebar
    header.add(&csd_button);
    win.set_titlebar(Some(&header));

    win.show_all();
}

fn main() {
    let app = Application::builder()
        .application_id("com.ezioleq.LilyClip")
        .build();

    app.connect_activate(build_ui);

    app.run();
}
