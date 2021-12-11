#![windows_subsystem = "windows"]

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

mod strings;

fn add_header_button_popover(button: &gtk::MenuButton) {
    let popover = gtk::Popover::new(Some(button));
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 4);

    let stats = gtk::Button::with_label("Stats");

    vbox.add(&stats);
    vbox.add(&gtk::Button::with_label("About"));
    vbox.add(&gtk::Button::with_label("Close"));
    vbox.show_all();

    popover.add(&vbox);
    popover.set_position(gtk::PositionType::Bottom);

    button.set_popover(Some(&popover));
}

fn main() {
    let app = Application::builder()
        .application_id("com.ezioleq.LilyClip")
        .build();

    app.connect_activate(|app| {
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(350)
            .default_height(400)
            .title(strings::TITLE)
            .build();

        // Header bar button for misc options
        let csd_button = gtk::MenuButton::new();
        let csd_button_img = gtk::Image::new();
        csd_button_img.set_from_icon_name(
            "open-menu-symbolic".into(),
            gtk::IconSize::Button
        );
        csd_button.set_image(Some(&csd_button_img));

        csd_button.connect_clicked(|_| {
            let clipboard = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
            clipboard.set_text("aaaaaa");
        });

        // Popover for header bar button
        add_header_button_popover(&csd_button);
        
        // CSD header bar
        let header = gtk::HeaderBar::builder()
            .title(strings::TITLE)
            .subtitle(strings::SUBTITLE)
            .has_subtitle(false)
            .show_close_button(true)
            .build();

        header.add(&csd_button);

        win.set_titlebar(Some(&header));
        win.show_all();
    });

    app.run();
}
