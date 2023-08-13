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
pub mod menu_manipulation;
use menu_manipulation::*;
pub mod file_manipulation;
use file_manipulation::*;
pub mod theme_and_decoration;

use fltk_decl::*;
use std::env;
use fltk::{menu, menu::*, app, prelude::MenuExt};

pub const OPERATING_SYSTEM: &str = env::consts::OS;
pub const APP_NAME: &str = "Wildfire Write";


pub struct Configuration {
    pub screen_dimension_w: u32,
    pub screen_dimension_h: u32,
    pub menu_bar_height: u32,
    pub side_bar_width: u32,
    pub side_bar_button_height: u32,
}



fn main() {
    println!("{}", OPERATING_SYSTEM);
    // portng to fltk decl something i should've done from the start
    DeclarativeApp::new_json(200, 300, "wildfire_write", "resources/gui/gui.json").run(|_win|
    {
        if let Some(mut menu_bar) = app::widget_from_id::<menu::Choice>("menu_bar") {
            menu_bar.add_choice("File|Edit|Preferences|License|File/Open");
        }

    }).unwrap();

    // all the widgets for the window
    /*let mut wind = Window::new(720, 1280, 1600, 900, APP_NAME);
    let mut menubar = SysMenuBar::new(0,0,1600, 25, APP_NAME);

    // open file
    let mut file_str:String = String::new();
    // menu bar stuff
    
    let app_clone = a.clone();
    add_to_menubar(menubar.clone(),app_clone);

    menubar.add(
        "File/Open\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        move |_|{
            file_str = open_file();
            println!("{}", file_str);
        },
    );

    // turn this into a code module

    module_ui(a.clone());
    
    */
    //Runs at application close
}
