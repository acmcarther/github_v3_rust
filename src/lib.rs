extern crate hyper;
extern crate time;
extern crate rustc_serialize;

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

mod commits;
mod github_client;
mod pull_requests;
mod repos;
mod types;
mod users;

pub use github_client::GithubClient;
