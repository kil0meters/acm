//! Toplevel views (pages) associated with Routes.

mod account;
mod home;
mod leaderboard;
mod login;
mod problem;
mod problem_editor;
mod problem_list;
mod signup;

pub use account::AccountView;
pub use home::HomeView;
pub use leaderboard::LeaderboardView;
pub use login::LoginView;
pub use problem::ProblemView;
pub use problem_editor::ProblemEditorView;
pub use problem_list::ProblemListView;
pub use signup::SignupView;
