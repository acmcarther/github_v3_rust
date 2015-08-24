pub mod types;
pub mod url_builders;

mod repos {
  use hyper::header::Scheme;
  use hyper::method::Method;

  use github_client::GithubClient;

  use std::any::Any;
  use std::io::ErrorKind;

  use types::{
    RepoName,
    BranchName,
    UserName,
    OrganizationName,
    Repository,
    Url,
    GitTm,
    GitErr,
  };

  use repos::types::{
    Repo,
    RepoPermissions,
    RepoQuery,
    CreateRepository,
    EditRepository,
    ContributorsQuery,
    LanguagePile,
    Team,
    Tag,
    Branch,
    FullBranch,
    DeletedStatus,
  };

  use repos::url_builders as url_builders;

  use commits::types::GithubCommit;
  use users::types::User;

  pub trait Repoer {
    fn list_own_repos(query: RepoQuery) -> Result<Vec<Repo>, GitErr>;
    fn list_user_repos(owner: UserName /*TODO: query: Option<UserRepoQuery>*/) -> Result<Vec<Repo>, GitErr>;
    fn list_org_repos(org: OrganizationName /*TODO: query: Option<OrgRepoQuery>*/) -> Result<Vec<Repo>, GitErr>;
    fn list_public_repos(last_seen: u32) -> Result<Vec<Repo>, GitErr>; // TODO: This Id
    fn create_own_repo(details: CreateRepository) -> Result<Repo, GitErr>;
    fn create_org_repo(org: OrganizationName, details: CreateRepository) -> Result<Repo, GitErr>;
    fn get_repo(repo: Repository) -> Result<Repo, GitErr>;
    fn edit_repo(repo: Repository, details: EditRepository) -> Result<Repo, GitErr>;
    fn list_contributors(repo: Repository, query: ContributorsQuery) -> Result<Vec<User>, GitErr>;
    fn list_languages(repo: Repository) -> Result<LanguagePile, GitErr>;
    fn list_teams(repo: Repository) -> Result<Vec<Team>, GitErr>;
    fn list_tags(repo: Repository) -> Result<Vec<Tag>, GitErr>;
    fn list_branches(repo: Repository) -> Result<Vec<Branch>, GitErr>;
    fn get_branch(repo: Repository, branch: BranchName) -> Result<FullBranch, GitErr>;
    fn delete_repo(repo: Repository) -> Result<DeletedStatus, GitErr>;
  }
}
