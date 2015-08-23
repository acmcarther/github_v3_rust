pub use self::types::{
  Commit,
  GithubCommit,
};

mod types {
  use types::{
    BranchName,
    Sha,
    Url,
    GitTm,
    Message,

  };

  use users::types::User;
  use repos::types::Repo;

  #[derive(RustcDecodable, Debug)]
  pub struct Commit {
    label: BranchName,
    // ref TODO: custom decoder for reserved word
    sha: Sha,
    user: User,
    repo: Repo,
  }

  #[derive(RustcDecodable, Debug)]
  pub struct GithubCommit {
    url: Url,
    sha: Sha,
    html_url: Url,
    comments_url: Url,
    commit: GithubCommitSummary,
    author: User,
    committer: User,
    parents: Vec<CommitTreeNode>
  }

  #[derive(RustcDecodable, Debug)]
  pub struct GithubCommitSummary {
    url: Url,
    author: CommitAuthor,
    committer: CommitAuthor,
    message: Message,
    tree: CommitTreeNode,
    comment_count: u32,
  }

  #[derive(RustcDecodable, Debug)]
  pub struct CommitAuthor {
    name: String,
    email: String,
    date: GitTm
  }

  #[derive(RustcDecodable, Debug)]
  pub struct CommitTreeNode {
    url: Url,
    sha: Sha,
  }
}
