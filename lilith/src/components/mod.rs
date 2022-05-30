//! Shared components used throughout the application.

mod code_editor;
mod error;
mod footer;
mod modal;
mod navbar;
mod tabbed;
mod tests;

pub use code_editor::CodeEditor;
pub use error::ErrorBox;
pub use footer::Footer;
pub use modal::Modal;
pub use navbar::Navbar;
pub use tabbed::Tabbed;
pub use tests::TestsEditor;
