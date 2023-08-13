use fltk::{
    app,
    prelude::*, 
    menu, menu::*, 
    enums::*,
    dialog::HelpDialog,
    window::*,
    group,
    button::*,
    frame::*,
    };
use crate::theme_and_decoration;
use theme_and_decoration::*;
use crate::file_manipulation::*;

fn menu_cb(m: &mut impl MenuExt){
    if let Some(choice) = m.choice() {
        match choice.as_str() {
            "New Note\t" => create_file(".md"),
            "Third" => println!("Third"),
            "Quit\t" => {
                println!("Quitting");
                app::quit();
            },
            _ => println!("{}", choice),
        }
    }
}

pub fn add_to_menubar(mut menubar: SysMenuBar, a:app::App) {
    menubar.add(
        "File/New Note\t",
        Shortcut::None,
        menu::MenuFlag::Normal, 
        menu_cb);

    menubar.add(
        "File/Quit\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        menu_cb,
    );
    menubar.add(
       "Edit/Insert\t", 
        Shortcut::None,
        menu::MenuFlag::Normal,
        menu_cb
    );
    menubar.add(
        "Help/About\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        |_| {
            let mut help = HelpDialog::new(0,0, 1600, 900);
            let _ =help.load("resources/help/main.html");
            help.show();

            while help.shown(){
                app::wait();
            }
        }
    );
    menubar.add(
        "Help/License\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        |_|{
            let mut help_license = HelpDialog::new(0,0, 1600, 900);
            let _ = help_license.load("resources/help/license.html");
            help_license.show();  
            while help_license.shown(){
                app::wait();
            }         
        }
    );
    menubar.add(
        "Preferences/Set Theme/GTK\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        move |_| {
            let app_clone = a.clone();
            theme_button_callbacks(app_clone, "GTK");
        }
    );

    menubar.add(
        "Preferences/Set Theme/Oxy\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        move |_| {
            let app_clone = a.clone();
            theme_button_callbacks(app_clone, "OXY");
        }
    );

    menubar.add(
        "Preferences/Set Theme/Base\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        move |_| {
            let app_clone = a.clone();
            theme_button_callbacks(app_clone, "BAS");
        }
    );

    menubar.add(
        "Preferences/Set Theme/Plastic\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        move |_| {
            let app_clone = a.clone();
            theme_button_callbacks(app_clone, "PLA");
        }
    );

    menubar.add(
        "Preferences/Set Theme/Gleam\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        move |_| {
            let app_clone = a.clone();
            theme_button_callbacks(app_clone, "GLE");
        }
    );
}

pub fn module_ui(a:app::App){
    let mut side_menu = Window::new(0, 25, 256, 875, "");
    side_menu.set_color(Color::Black);
    let mut side_menu_opt = group::Pack::default_fill();
    side_menu_opt.set_spacing(0);

    let module_name: [&str; 2] = ["Characters", "World Timeline(NYI)"];
    let mut character_module_button = Button::new(0,25, 256, 50, module_name[0]);
    let mut timeline_module_button = Button::new(0,25, 256, 50, module_name[1]);

    side_menu.end();
    
    //The module window

    let mut character_module_menu = Window::new(256,25, 1344, 825, "");
    character_module_menu.set_color(Color::Magenta);
    character_module_menu.begin();

    // Character module UI
    let mut cm_side_bar = Window::new(0, 0, 192, 825, "");
    cm_side_bar.begin();
    let mut cm_side_bar_pack = group::Pack::default_fill();
    let mut open_character_note_button = Button::new(0,0, 192, 60, "Open Character .json");
    let _cm_side_bar_frame = Frame::new(0, 60, 192, 60, "");
    

    side_menu_opt.set_spacing(0);
    cm_side_bar.end();

    cm_side_bar.hide();
    open_character_note_button.hide();

    // this chunk of code allows for opening of character jsons
    let mut character_file_to_open:String = String::new();
    open_character_note_button.set_callback(move|_| {

        character_file_to_open = open_file();
        println!("{}", character_file_to_open);
    });

    // Cloning the button to create independent copies for each closure
    let mut open_character_note_button_show = open_character_note_button.clone();
    let mut cm_side_bar_show = cm_side_bar.clone();
    // Using the cloned button in the character_module_button callback
    character_module_button.set_callback(move |_|{
        open_character_note_button_show.show();
        cm_side_bar_show.show();
    });

    let mut open_character_note_button_hide_0 = open_character_note_button.clone();
    let mut cm_side_bar_hide_0 = cm_side_bar.clone();
    timeline_module_button.set_callback(move |_|{
        open_character_note_button_hide_0.hide();
        cm_side_bar_hide_0.hide();    
    });

    character_module_menu.end();
    
}