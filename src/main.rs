#![windows_subsystem = "windows"]

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

mod strings;
mod ui;
mod data;

/// Adds a [Popover](gtk::Popover) menu to the header bar's [MenuButton](gtk::MenuButton)
fn add_header_button_popover(button: &gtk::MenuButton) {
    let popover = gtk::Popover::new(Some(button));
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 4);

    // Stats button
    let stats = gtk::ModelButton::builder()
        .label("Stats")
        .image(&gtk::Image::from_icon_name(
            "document-properties".into(),
            gtk::IconSize::Button,
        ))
        .build();

    // About button
    let about = gtk::ModelButton::builder()
        .label("About")
        .image(&gtk::Image::from_icon_name(
            "help-about".into(),
            gtk::IconSize::Button,
        ))
        .build();

    // Show about window
    about.connect_clicked(ui::show_about_dialog);

    // Exit button
    let close = gtk::ModelButton::builder()
        .label("Close")
        .image(&gtk::Image::from_icon_name(
            "window-close".into(),
            gtk::IconSize::Button,
        ))
        .build();

    // Exit button clicked
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

/// Builds UI elements
fn build_ui(app: &Application) {
    let win = ApplicationWindow::builder()
        .application(app)
        .default_width(350)
        .default_height(400)
        .title(strings::TITLE)
        .build();

    // Header bar's button
    let header_button = ui::create_menu_button_with_icon("open-menu-symbolic");
    add_header_button_popover(&header_button);

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
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let search_bar = gtk::SearchEntry::new();
    vbox.add(&search_bar);

    let scroll = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scroll.set_vexpand(true);

    let listbox = gtk::ListBox::new();
    for i in 1..=50 {
        listbox.add(&gtk::Label::new(Some(format!("Element {}", i).as_str())));
    }
    listbox.show_all();

    let man_box = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
    man_box.set_layout_style(gtk::ButtonBoxStyle::Expand);
    man_box.add(&ui::create_button_with_icon("list-add"));
    man_box.add(&ui::create_button_with_icon("list-remove"));

    let move_box = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
    move_box.set_layout_style(gtk::ButtonBoxStyle::Expand);
    move_box.add(&ui::create_button_with_icon("go-up"));
    move_box.add(&ui::create_button_with_icon("go-down"));

    let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
    separator.set_hexpand(true);

    let action_bar = gtk::ActionBar::new();
    action_bar.add(&man_box);
    action_bar.add(&move_box);
    action_bar.add(&separator);
    action_bar.add(&ui::create_button_with_icon("edit-copy"));

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
