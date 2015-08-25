extern crate hyper;
extern crate time;
extern crate rustc_serialize;

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

#[macro_use]
pub mod types;

mod commits;
mod github_client;
mod pull_requests;
mod commit_comments;
mod issue_comments;
mod repos;

mod users;

pub use github_client::GithubClient;

pub use commit_comments::types as commit_comment_types;
pub use commit_comments::CommitCommenter;

pub use issue_comments::types as issue_comment_types;
pub use issue_comments::IssueCommenter;

pub use pull_requests::types as pull_request_types;
pub use pull_requests::PullRequester;

pub use repos::types as repo_types;
pub use repos::Repoer;
