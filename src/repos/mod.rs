pub mod url_builders;

use hyper::method::Method;

use github_client::{SimpleClient};

use std::io::ErrorKind;

use types::{
  BranchName,
  UserName,
  OrganizationName,
  Repository,
  GitErr,
  Repo,
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
  PublicReposQuery,
  GithubUser
};

pub trait Repoer {
  fn list_own_repos(&self, query: RepoQuery) -> Result<Vec<Repo>, GitErr>;
  fn list_user_repos(&self, owner: UserName /*TODO: query: Option<UserRepoQuery>*/) -> Result<Vec<Repo>, GitErr>;
  fn list_org_repos(&self, org: OrganizationName /*TODO: query: Option<OrgRepoQuery>*/) -> Result<Vec<Repo>, GitErr>;
  fn list_public_repos(&self, query: Option<PublicReposQuery>) -> Result<Vec<Repo>, GitErr>;
  fn create_own_repo(&self, details: CreateRepository) -> Result<Repo, GitErr>;
  fn create_org_repo(&self, org: OrganizationName, details: CreateRepository) -> Result<Repo, GitErr>;
  fn get_repo(&self, repo: Repository) -> Result<Repo, GitErr>;
  fn edit_repo(&self, repo: Repository, details: EditRepository) -> Result<Repo, GitErr>;
  fn list_contributors(&self, repo: Repository, query: ContributorsQuery) -> Result<Vec<GithubUser>, GitErr>;
  fn list_languages(&self, repo: Repository) -> Result<LanguagePile, GitErr>;
  fn list_teams(&self, repo: Repository) -> Result<Vec<Team>, GitErr>;
  fn list_tags(&self, repo: Repository) -> Result<Vec<Tag>, GitErr>;
  fn list_branches(&self, repo: Repository) -> Result<Vec<Branch>, GitErr>;
  fn get_branch(&self, repo: Repository, branch: BranchName) -> Result<FullBranch, GitErr>;
  fn delete_repo(&self, repo: Repository) -> Result<DeletedStatus, GitErr>;
}

impl<C: SimpleClient> Repoer for C {
  fn list_own_repos(&self, query: RepoQuery) -> Result<Vec<Repo>, GitErr> {
    let url = url_builders::own_repos();
    self.request_with_payload(Method::Get, url, query)
  }

  fn list_user_repos(&self, owner: UserName /*TODO: query: Option<UserRepoQuery>*/) -> Result<Vec<Repo>, GitErr> {
    let url = url_builders::user_repos(&owner);
    self.request_without_payload(Method::Get, url)
  }

  fn list_org_repos(&self, org: OrganizationName /*TODO: query: Option<OrgRepoQuery>*/) -> Result<Vec<Repo>, GitErr> {
    let url = url_builders::organization_repos(&org);
    self.request_without_payload(Method::Get, url)
  }

  fn list_public_repos(&self, query: Option<PublicReposQuery>) -> Result<Vec<Repo>, GitErr> {
    let url = url_builders::all_repos();
    match query {
      Some(query) => self.request_with_payload(Method::Get, url, query),
      None => self.request_without_payload(Method::Get, url)
    }
  }

  fn create_own_repo(&self, details: CreateRepository) -> Result<Repo, GitErr> {
    let url = url_builders::own_repos();
    self.request_with_payload(Method::Post, url, details)
  }

  fn create_org_repo(&self, org: OrganizationName, details: CreateRepository) -> Result<Repo, GitErr> {
    let url = url_builders::organization_repos(&org);
    self.request_with_payload(Method::Post, url, details)
  }

  fn get_repo(&self, repo: Repository) -> Result<Repo, GitErr> {
    let url = url_builders::repo_at(&repo);
    self.request_without_payload(Method::Get, url)
  }

  fn edit_repo(&self, repo: Repository, details: EditRepository) -> Result<Repo, GitErr> {
    let url = url_builders::repo_at(&repo);
    self.request_with_payload(Method::Patch, url, details)
  }

  fn list_contributors(&self, repo: Repository, query: ContributorsQuery) -> Result<Vec<GithubUser>, GitErr> {
    let url = url_builders::contributors_at(&repo);
    self.request_with_payload(Method::Get, url, query)
  }

  fn list_languages(&self, repo: Repository) -> Result<LanguagePile, GitErr> {
    let url = url_builders::languages_at(&repo);
    self.request_without_payload(Method::Get, url)
  }

  fn list_teams(&self, repo: Repository) -> Result<Vec<Team>, GitErr> {
    let url = url_builders::teams_at(&repo);
    self.request_without_payload(Method::Get, url)
  }

  fn list_tags(&self, repo: Repository) -> Result<Vec<Tag>, GitErr> {
    let url = url_builders::tags_at(&repo);
    self.request_without_payload(Method::Get, url)
  }

  fn list_branches(&self, repo: Repository) -> Result<Vec<Branch>, GitErr> {
    let url = url_builders::branches_at(&repo);
    self.request_without_payload(Method::Get, url)
  }

  fn get_branch(&self, repo: Repository, branch: BranchName) -> Result<FullBranch, GitErr> {
    let url = url_builders::branch_at(&repo, &branch);
    self.request_without_payload(Method::Get, url)
  }

  #[allow(unused_variables)]
  fn delete_repo(&self, repo: Repository) -> Result<DeletedStatus, GitErr> {
    Err(GitErr::new(ErrorKind::Other, "not implemented".to_owned()))
  }
}
