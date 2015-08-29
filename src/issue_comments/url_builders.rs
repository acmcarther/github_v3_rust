use types::{
  Url,
  IssueId,
};

use types::repos::Repository;

use types::comments::CommentId;

#[allow(dead_code)]
pub fn issue_comments(repo: &Repository, issue_id: &IssueId) -> Url {
  "https://api.github.com/repos/".to_owned() +
    &repo.owner + "/" +
    &repo.repo_name + "/issues/" +
    &issue_id.to_string() + "/comments"
}

pub fn issue_comments_for_repo(repo: &Repository) -> Url {
  "https://api.github.com/repos/".to_owned() +
    &repo.owner + "/" +
    &repo.repo_name + "/issues/comments"
}

#[allow(dead_code)]
pub fn issue_comment_at(repo: &Repository, comment_id: &CommentId) -> Url {
  issue_comments_for_repo(repo) + "/" + &comment_id.to_string()
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
    issue_comments,
    issue_comments_for_repo,
    issue_comment_at,
  };

  #[test]
  fn it_builds_issue_comments() {
    let repo = Repository { owner: "test_owner".to_owned(), repo_name: "test_repo".to_owned() };
    let issue_id = 21;
    let expected = "https://api.github.com/repos/test_owner/test_repo/issues/21/comments".to_owned();
    expect!(issue_comments(&repo, &issue_id)).to(be_equal_to(expected));
  }

  #[test]
  fn it_builds_issue_comments_for_repo() {
    let repo = Repository { owner: "test_owner".to_owned(), repo_name: "test_repo".to_owned() };
    let expected = "https://api.github.com/repos/test_owner/test_repo/issues/comments".to_owned();
    expect!(issue_comments_for_repo(&repo)).to(be_equal_to(expected));
  }

  #[test]
  fn it_builds_issue_comment_at() {
    let repo = Repository { owner: "test_owner".to_owned(), repo_name: "test_repo".to_owned() };
    let comment_id = 21;
    let expected = "https://api.github.com/repos/test_owner/test_repo/issues/comments/21".to_owned();
    expect!(issue_comment_at(&repo, &comment_id)).to(be_equal_to(expected));
  }
}
