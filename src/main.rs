use fltk::{prelude::*, * };
use fltk_decl::DeclarativeApp;

// path to window resources
const PATH: &str = "resources/gui.json";


fn main() {
    DeclarativeApp::new_json(1200, 720, "MyApp", PATH)
        .run(|_win| {
            if let Some(mut choice) = app::widget_from_id::<menu::SysMenuBar>("menubar") {
                choice.add_choice("File|Edit|Help|Module");
                choice.set_callback(|c| {
                    if let Some(mut label) = app::widget_from_id::<frame::Frame>("label") {
                        label.set_label(&format!("{:?}", c.choice()));
                    }
                });
            }
        })
        .unwrap();
    
}