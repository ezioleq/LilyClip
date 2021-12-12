#![windows_subsystem = "windows"]

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

mod strings;

/// Adds a [Popover](gtk::Popover) menu to the header bar's [MenuButton](gtk::MenuButton)
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

    // Add those buttons to the vertical box layout
    vbox.add(&stats);
    vbox.add(&about);
    vbox.add(&close);
    vbox.show_all();

    popover.add(&vbox);
    popover.set_position(gtk::PositionType::Bottom);

    // Set button's popover
    button.set_popover(Some(&popover));
}

/// Creates a [MenuButton](gtk::MenuButton) for the [header bar](gtk::HeaderBar)
fn create_header_button() -> gtk::MenuButton {
    let button = gtk::MenuButton::new();
    let button_img = gtk::Image::new();

    button_img.set_from_icon_name("open-menu-symbolic".into(), gtk::IconSize::Button);
    button.set_image(Some(&button_img));

    // Add menu popover
    add_header_button_popover(&button);

    button
}

/// Builds UI elements
fn build_ui(app: &Application) {
    let win = ApplicationWindow::builder()
        .application(app)
        .default_width(350)
        .default_height(400)
        .title(strings::TITLE)
        .build();

    let header_button = create_header_button();

    // CSD header bar
    let header = gtk::HeaderBar::builder()
        .title(strings::TITLE)
        .subtitle(strings::SUBTITLE)
        .has_subtitle(false)
        .show_close_button(true)
        .build();

    // Add and set the new titlebar
    header.add(&header_button);
    win.set_titlebar(Some(&header));

    // Layout
    let scroll = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scroll.set_vexpand(true);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let listbox = gtk::ListBox::new();
    for i in 1..=50 {
        listbox.add(&gtk::Label::new(Some(format!("Element {}", i).as_str())));
    }
    listbox.show_all();

    let action_bar = gtk::ActionBar::new();
    action_bar.add(&gtk::Button::with_label("test"));
    action_bar.add(&gtk::Button::with_label("test2"));
    action_bar.show_all();


    scroll.add(&listbox);
    vbox.add(&scroll);
    vbox.add(&action_bar);
    win.add(&vbox);

    win.show_all();
}

/// Program entry point
fn main() {
    let app = Application::builder()
        .application_id("com.ezioleq.LilyClip")
        .build();

    app.connect_activate(build_ui);

    app.run();
}
