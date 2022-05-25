use monaco::{sys::editor::{IStandaloneEditorConstructionOptions, BuiltinTheme}, api::{CodeEditorOptions, TextModel}};
use web_sys::window;
use std::rc::Rc;


pub fn is_darkmode() -> bool {
    let window = window().unwrap();

    window.match_media("(prefers-color-scheme: dark)").unwrap().unwrap().matches()
}

pub fn themed_editor_with_model(model: TextModel) -> IStandaloneEditorConstructionOptions {
    let options = {
        let builder = CodeEditorOptions::default().with_model(model);

        let builder = if is_darkmode() {
            builder.with_builtin_theme(BuiltinTheme::VsDark)
        } else {
            builder
        };

        Rc::new(builder).to_sys_options()
    };

    options.set_font_size(Some(18.0));
    options.set_automatic_layout(Some(true));

    options
}
