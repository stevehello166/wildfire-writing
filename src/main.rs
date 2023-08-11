/*
Wildfire Write, a free and opensource writing software made for writers, inspired by campfire write and world anvil.
(C) 2023
Nadichamp
This program is free software: you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation, either version 3 of the License, or
        (at your option) any later version.
    
        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.
    
        You should have received a copy of the GNU General Public License
        along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/



// the Import block
mod file_manipulation;
use file_manipulation::*;

use fltk::{
    app,
    prelude::*,
    window::Window, 
    menu, menu::*, 
    enums::*,
    button::Button,
    dialog::HelpDialog,
    group,
    dialog,
    frame::*
    };
use std::env;

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
        "Help/How To Use\t",
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
    menubar.add(
        "Help/License\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        |_|{
            let mut help_license = HelpDialog::new(0,0, 1600, 900);
            help_license.load("resources/help/liscense.html");
            help_license.show();  
            while help_license.shown(){
                app::wait();
            }         
        }
    );
    let app_clone = a.clone();
    menubar.add(
        "Preferences/Set Theme/GTK\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        move|_| {
            app_clone.with_scheme(app::Scheme::Gtk);
        }
    );
    let app_clone = a.clone();
    menubar.add(
        "Preferences/Set Theme/Oxy\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        move|_| {
            app_clone.with_scheme(app::Scheme::Oxy);
        }
    );

    let mut side_menu = Window::new(0, 25, 256, 875, "");
    side_menu.set_color(Color::Black);
    let mut side_menu_opt = group::Pack::default_fill();
    side_menu_opt.set_spacing(0);

    // originally i was going to write a highly complex block of code here to automate the creation of module buttons, however this is easier
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
    let mut cm_side_bar_frame = Frame::new(0, 60, 192, 60, "");
    

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
    

    wind.end();
    wind.show();
    a.run().unwrap();
    //Runs at application close
}