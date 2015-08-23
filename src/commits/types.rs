pub use self::types::{
  Commit
};

mod types {
  use types::{
    BranchName,
    Sha,
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
}
