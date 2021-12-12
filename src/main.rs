#![windows_subsystem = "windows"]

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

mod strings;
mod ui;

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
    let button = ui::create_menu_button_with_icon("open-menu-symbolic");

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

    let a = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
    a.set_layout_style(gtk::ButtonBoxStyle::Expand);
    a.add(&ui::create_button_with_icon("list-add"));
    a.add(&ui::create_button_with_icon("list-remove"));

    let b = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
    b.set_layout_style(gtk::ButtonBoxStyle::Expand);
    b.add(&ui::create_button_with_icon("go-up"));
    b.add(&ui::create_button_with_icon("go-down"));
    
    let action_bar = gtk::ActionBar::new();
    action_bar.add(&a);
    action_bar.add(&b);

    action_bar.set_hexpand(true);

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
