/*
Wildfire Write, a free and open source writing software to keep your worldbuilding organized, inspired by campfire write and world anvil.
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
mod theme_and_decoration;
use file_manipulation::*;
use theme_and_decoration::*;

use fltk::{prelude::*, *, enums::*,dialog::*};
use fltk_decl::{DeclarativeApp,Widget};
use std::env;

pub const OPERATING_SYSTEM: &str = env::consts::OS;
pub const APP_NAME: &str = "Wildfire Write";

fn menu_cb(m: &mut impl MenuExt){
    if let Some(choice) = m.choice() {
        match choice.as_str() {
            "New Note\t" => create_file(".md"),
            "Insert" => println!("Insert"),// this will eventually be able to insert images and other things into documents
            "Quit\t" => {
                println!("Quitting");
                app::quit();
            },
            _ => println!("{}", choice),
        }
    }
}
fn main() {
    DeclarativeApp::new_json(1280, 720, "Wildfire Write", "resources/gui/gui.json").run(|_win| 
        {
            if let Some (mut wind) = app::widget_from_id::<window::DoubleWindow>("wind"){
            wind.set_color(Color::Black)
            }
            if let Some (mut menubar) = app::widget_from_id::<menu::SysMenuBar>("menu_bar"){
                //menubar.add("template/example\t", Shortcut::None, menu::MenuFlag::Normal, menu_cb);
                menubar.add("File/New Note\t", Shortcut::None, menu::MenuFlag::Normal, menu_cb);  
                menubar.add("File/Quit\t", Shortcut::None, menu::MenuFlag::Normal, menu_cb);  
                menubar.add("File/Open\t", Shortcut::None, menu::MenuFlag::Normal, |_|{
                    let file_to_open = open_file();
                });  
                menubar.add("Edit/Insert\t", Shortcut::None,menu::MenuFlag::Normal,menu_cb);  
                /*menubar.add("Preferences/Set Theme/GTK\t",Shortcut::None,menu::MenuFlag::Normal,move |_| {
                    let app_clone = a.clone();
                    theme_button_callbacks(app_clone, "GTK");
                });
                waiting until a way to implement themes is added  to fltk-decl*/    
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
                    });
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
            }
        }).unwrap();
}
