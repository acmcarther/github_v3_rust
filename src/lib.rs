extern crate hyper;
extern crate time;
extern crate rustc_serialize;

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

#[macro_use]
pub mod types;

pub mod github_client;
mod commits;
mod pull_requests;
mod commit_comments;
mod issue_comments;
mod repos;
mod events;
mod users;

pub use commit_comments::CommitCommenter;
pub use issue_comments::IssueCommenter;
pub use pull_requests::PullRequester;
pub use repos::Repoer;

pub use commit_comments::types as commit_comments_types;
pub use issue_comments::types as issue_comments_types;
pub use pull_requests::types as pull_requests_types;
pub use repos::types as repos_types;
pub use users::types as users_types;
pub use events::types as events_types;
pub use commits::types as commits_types;

pub use hyper::header::Authorization;
pub use hyper::method::Method;
