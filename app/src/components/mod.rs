//! Shared components used throughout the application.

mod error;
mod modal;
mod navbar;
mod tabbed;
mod tests;

pub use error::ErrorBox;
pub use modal::Modal;
pub use navbar::Navbar;
pub use tabbed::Tabbed;
pub use tests::TestsEditor;
