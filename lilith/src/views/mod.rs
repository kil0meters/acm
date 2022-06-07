//! Toplevel views (pages) associated with Routes.

mod account;
mod home;
mod leaderboard;
mod logout;
mod meeting_editor;
mod meetings;
mod problem;
mod problem_editor;
mod problem_list;
mod signup;
mod submission;

pub use account::AccountView;
pub use home::HomeView;
pub use leaderboard::LeaderboardView;
pub use logout::LogoutView;
pub use meeting_editor::MeetingEditorView;
pub use meetings::MeetingsView;
pub use problem::{ProblemView, ProblemViewInner};
pub use problem_editor::ProblemEditorView;
pub use problem_list::ProblemListView;
pub use signup::{LoginView, SignupView};
pub use submission::SubmissionView;
