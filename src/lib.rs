#![feature(plugin)]
#![plugin(stainless)]

extern crate hyper;
extern crate time;
extern crate rustc_serialize;

// TODO: Make this only a dev dependency
extern crate stainless;

mod commits;
mod github_client;
mod pull_requests;
mod repos;
mod types;
mod users;

pub use github_client::GithubClient;
