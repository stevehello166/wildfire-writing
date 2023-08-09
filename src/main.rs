// the Import block
mod file_manipulation;
use file_manipulation::*;

use fltk::{
    app,
    prelude::*,
    window::Window, 
    menu, menu::*, 
    enums::*,
    button::{Button, self},
    dialog::HelpDialog,
    group,
    dialog
    };
use std::{env, thread};

pub const OPERATING_SYSTEM: &str = env::consts::OS;
pub const APP_NAME: &str = "Wildfire Write";


pub struct Configuration {
    pub screen_dimension_w: u32,
    pub screen_dimension_h: u32,
    pub menu_bar_height: u32,
    pub side_bar_width: u32,
    pub side_bar_button_height: u32,
}

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

fn main() {
    println!("{}", OPERATING_SYSTEM);
    let a = app::App::default();
    dialog::message_title_default("Wildfire Write");
    // all the widgets for the window
    let mut wind = Window::new(720, 1280, 1600, 900, APP_NAME);
    let mut menubar = SysMenuBar::new(0,0,1600, 25, APP_NAME);

    // open file
    let mut file_str:String = "".to_string();
    // menu bar stuff
    menubar.add(
        "File/New Note\t",
        Shortcut::None,
        menu::MenuFlag::Normal, 
        menu_cb);
    menubar.add(
        "File/Open\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        move |_|{
            file_str = open_file();
            println!("{}", file_str);
        },
    );
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
        "Help\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        |_| {
            let mut help = HelpDialog::new(0,0, 1600, 900);
            help.load("resources/help/main.html");
            help.show();

            while help.shown(){
                app::wait();
            }
        }
    );


    let mut side_menu = Window::new(0, 25, 256, 875, "");
    side_menu.set_color(Color::Black);
    let mut side_menu_opt = group::Pack::default_fill();
    side_menu_opt.set_spacing(0);

    // originally i was going to write a highly complex block of code here to automate the creation of module buttons, however this is easier
    let module_name: [&str; 2] = ["Character Module", "Timeline_Module"];
    let mut character_module_button = Button::new(0,25, 256, 50, module_name[0]);
    let mut timeline_module_button = Button::new(0,25, 256, 50, module_name[1]);

    side_menu.end();
    
    //The module window

    let mut character_module_menu = Window::new(256,25, 1344, 825, "");
    character_module_menu.set_color(Color::Magenta);
    character_module_menu.begin();

    let mut open_character_note_button = Button::new(0,0, 192, 64, "Open Character .json");
    open_character_note_button.hide();

    // this chunk of code allows for opening of character jsons
    let mut character_file_to_open:String = String::new();
    open_character_note_button.set_callback(move|_| {
        character_file_to_open = open_file();
        println!("{}", character_file_to_open)
    });

    // Cloning the button to create independent copies for each closure
    let mut open_character_note_button_show = open_character_note_button.clone();
    // Using the cloned button in the character_module_button callback
    character_module_button.set_callback(move |_|{
        open_character_note_button_show.show();
    });

    let mut open_character_note_button_hide_0 = open_character_note_button.clone();
    timeline_module_button.set_callback(move |_|{
        open_character_note_button_hide_0.hide();            
    });

    character_module_menu.end();
    

    wind.end();
    wind.show();
    a.run().unwrap();
    //Runs at application close
}