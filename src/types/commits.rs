use types::{
  BranchName,
  Sha,
  GithubUser,
  Message,
  GitTm,
  Filename,
  Url,
  GitUser
};

use types::repos::Repo;

#[derive(RustcDecodable, Debug)]
pub struct Commit {
  pub label: BranchName,
  // ref TODO: custom decoder for reserved word
  pub sha: Sha,
  pub user: GithubUser,
  pub repo: Repo,
}

#[derive(RustcDecodable, Debug)]
pub struct GithubCommit {
  pub url: Url,
  pub sha: Sha,
  pub html_url: Url,
  pub comments_url: Url,
  pub commit: GithubCommitSummary,
  pub author: GithubUser,
  pub committer: GithubUser,
  pub parents: Vec<CommitTreeNode>
}

#[derive(RustcDecodable, Debug)]
pub struct GithubCommitSummary {
  pub url: Url,
  pub author: CommitAuthor,
  pub committer: CommitAuthor,
  pub message: Message,
  pub tree: CommitTreeNode,
  pub comment_count: u32,
}

#[derive(RustcDecodable, Debug)]
pub struct CommitAuthor {
  pub name: String,
  pub email: String,
  pub date: GitTm
}

#[derive(RustcDecodable, Debug)]
pub struct CommitTreeNode {
  pub url: Url,
  pub sha: Sha,
}

#[derive(RustcDecodable, Debug)]
pub struct PushCommit {
  pub id: Sha,
  pub distinct: bool,
  pub message: Message,
  pub timestamp: GitTm,
  pub url: Url,
  pub author: GitUser,
  pub committer: GitUser,
  pub added: Vec<Filename>,
  pub removed: Vec<Filename>,
  pub modified: Vec<Filename>,
}

