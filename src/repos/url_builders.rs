use types::{
  OrganizationName,
  BranchName,
  Url
};
use types::users::UserName;
use types::repos::Repository;

#[allow(dead_code)]
pub fn own_repos() -> Url {
  "https://api.github.com/user/repos".to_owned()
}

#[allow(dead_code)]
pub fn user_repos(user: &UserName) -> Url {
  "https://api.github.com/users/".to_owned() +
    &user +
    "/repos"
}

#[allow(dead_code)]
pub fn organization_repos(org: &OrganizationName) -> Url {
  "https://api.github.com/orgs/".to_owned() +
    &org +
    "/repos"
}

#[allow(dead_code)]
pub fn all_repos() -> Url {
  "https://api.github.com/repositories".to_owned()
}

pub fn repo_at(repo: &Repository) -> Url {
  "https://api.github.com/repos/".to_owned() +
    &repo.owner +"/" +
    &repo.repo_name
}

#[allow(dead_code)]
pub fn contributors_at(repo: &Repository) -> Url {
  repo_at(repo) + "/contributors"
}

#[allow(dead_code)]
pub fn languages_at(repo: &Repository) -> Url {
  repo_at(repo) + "/languages"
}

#[allow(dead_code)]
pub fn teams_at(repo: &Repository) -> Url {
  repo_at(repo) + "/teams"
}

#[allow(dead_code)]
pub fn tags_at(repo: &Repository) -> Url {
  repo_at(repo) + "/tags"
}

pub fn branches_at(repo: &Repository) -> Url {
  repo_at(repo) + "/branches"
}

#[allow(dead_code)]
pub fn branch_at(repo: &Repository, branch: &BranchName) -> Url {
  branches_at(repo) + "/" + branch
}

#[cfg(test)]
mod tests {
  use expectest::core::expect;
  use expectest::matchers::be_equal_to;

  pub use types::{
    Url,
  };

  pub use types::repos::Repository;

  pub use super::{
    own_repos,
    user_repos,
    organization_repos,
    all_repos,
    repo_at,
    contributors_at,
    languages_at,
    teams_at,
    tags_at,
    branches_at,
    branch_at,
  };

  #[test]
  fn it_builds_own_repos() {
    let expected = "https://api.github.com/user/repos".to_owned();
    expect!(own_repos()).to(be_equal_to(expected));
  }

  #[test]
  fn it_builds_user_repos() {
    let expected = "https://api.github.com/users/test_owner/repos".to_owned();
    expect!(user_repos(&("test_owner".to_owned()))).to(be_equal_to(expected));
  }

  #[test]
  fn it_builds_organization_repos() {
    let expected = "https://api.github.com/orgs/test_org/repos".to_owned();
    expect!(organization_repos(&("test_org".to_owned()))).to(be_equal_to(expected));
  }

  #[test]
  fn it_builds_all_repos() {
    let expected = "https://api.github.com/repositories".to_owned();
    expect!(all_repos()).to(be_equal_to(expected));
  }

  #[test]
  fn it_builds_repo_at() {
    let repo = Repository { owner: "test_owner".to_owned(), repo_name: "test_repo".to_owned() };
    let expected = "https://api.github.com/repos/test_owner/test_repo".to_owned();
    expect!(repo_at(&repo)).to(be_equal_to(expected));
  }

  #[test]
  fn it_builds_contributors_at() {
    let repo = Repository { owner: "test_owner".to_owned(), repo_name: "test_repo".to_owned() };
    let expected = "https://api.github.com/repos/test_owner/test_repo/contributors".to_owned();
    expect!(contributors_at(&repo)).to(be_equal_to(expected));
  }

  #[test]
  fn it_builds_languages_at() {
    let repo = Repository { owner: "test_owner".to_owned(), repo_name: "test_repo".to_owned() };
    let expected = "https://api.github.com/repos/test_owner/test_repo/languages".to_owned();
    expect!(languages_at(&repo)).to(be_equal_to(expected));
  }

  #[test]
  fn it_builds_teams_at() {
    let repo = Repository { owner: "test_owner".to_owned(), repo_name: "test_repo".to_owned() };
    let expected = "https://api.github.com/repos/test_owner/test_repo/teams".to_owned();
    expect!(teams_at(&repo)).to(be_equal_to(expected));
  }

  #[test]
  fn it_builds_tags_at() {
    let repo = Repository { owner: "test_owner".to_owned(), repo_name: "test_repo".to_owned() };
    let expected = "https://api.github.com/repos/test_owner/test_repo/tags".to_owned();
    expect!(tags_at(&repo)).to(be_equal_to(expected));
  }

  #[test]
  fn it_builds_branches_at() {
    let repo = Repository { owner: "test_owner".to_owned(), repo_name: "test_repo".to_owned() };
    let expected = "https://api.github.com/repos/test_owner/test_repo/branches".to_owned();
    expect!(branches_at(&repo)).to(be_equal_to(expected));
  }

  #[test]
  fn it_builds_branch_at() {
    let repo = Repository { owner: "test_owner".to_owned(), repo_name: "test_repo".to_owned() };
    let expected = "https://api.github.com/repos/test_owner/test_repo/branches/test_branch".to_owned();
    expect!(branch_at(&repo, &("test_branch".to_owned()))).to(be_equal_to(expected));
  }
}
