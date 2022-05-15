//! Toplevel views (pages) associated with Routes.

pub mod home;
pub mod leaderboard;
pub mod login;
pub mod problem;
pub mod problem_editor;
pub mod problem_list;
pub mod signup;

pub use home::HomeView;
pub use leaderboard::LeaderboardView;
pub use login::LoginView;
pub use problem::ProblemView;
pub use problem_editor::ProblemEditorView;
pub use problem_list::ProblemListView;
pub use signup::SignupView;
