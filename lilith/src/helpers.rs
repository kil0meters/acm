use acm::models::{Auth, Session, User};
use monaco::{
    api::{CodeEditorOptions, TextModel},
    sys::editor::{BuiltinTheme, IEditorMinimapOptions, IStandaloneEditorConstructionOptions},
};
use pulldown_cmark::{html, Parser};
use std::rc::Rc;
use web_sys::window;

pub fn is_darkmode() -> bool {
    let window = window().unwrap();

    window
        .match_media("(prefers-color-scheme: dark)")
        .unwrap()
        .unwrap()
        .matches()
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

    let minimap_options = IEditorMinimapOptions::default();
    minimap_options.set_enabled(Some(false));
    options.set_minimap(Some(&minimap_options));

    options
}

pub fn parse_markdown(content: &str) -> String {
    let parser = Parser::new(&content);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

pub fn is_officer(session: &Option<Session>) -> bool {
    // If logged in and of sufficient rank
    if let Some(Session {
        user: User {
            auth: Auth::OFFICER | Auth::ADMIN,
            ..
        },
        ..
    }) = *session
    {
        true
    } else {
        false
    }
}
