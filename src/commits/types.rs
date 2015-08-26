pub use self::types::{
  Commit,
  GithubCommit,
  CommitTreeNode,
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
    pub label: BranchName,
    // ref TODO: custom decoder for reserved word
    pub sha: Sha,
    pub user: User,
    pub repo: Repo,
  }

  #[derive(RustcDecodable, Debug)]
  pub struct GithubCommit {
    pub url: Url,
    pub sha: Sha,
    pub html_url: Url,
    pub comments_url: Url,
    pub commit: GithubCommitSummary,
    pub author: User,
    pub committer: User,
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
}
