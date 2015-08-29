use types::{
  Url,
};


use types::comments::CommentId;

use types::repos::Repository;

use types::pull_requests::PullRequestId;

use pull_requests::url_builders::{
  pull_request_at,
  pull_requests,
};

pub fn pull_request_comments(repo: &Repository, pr_id: &PullRequestId) -> Url {
  pull_request_at(repo, pr_id) + "/comments"
}

pub fn all_pull_request_comments(repo: &Repository) -> Url {
  pull_requests(repo) + "/comments"
}

#[allow(dead_code)]
pub fn pull_request_comment_at(repo: &Repository, comment_id: &CommentId) -> Url {
  all_pull_request_comments(repo) + "/" + &comment_id.to_string()
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
    pull_request_comments,
    all_pull_request_comments,
    pull_request_comment_at,
  };

  #[test]
  fn it_builds_pull_request_comments() {
    let repo = Repository { owner: "test_owner".to_owned(), repo_name: "test_repo".to_owned() };
    let pr_id = 21;
    let expected = "https://api.github.com/repos/test_owner/test_repo/pulls/21/comments".to_owned();
    expect!(pull_request_comments(&repo, &pr_id)).to(be_equal_to(expected));
  }

  #[test]
  fn it_builds_all_pull_request_comments() {
    let repo = Repository { owner: "test_owner".to_owned(), repo_name: "test_repo".to_owned() };
    let expected = "https://api.github.com/repos/test_owner/test_repo/pulls/comments".to_owned();
    expect!(all_pull_request_comments(&repo)).to(be_equal_to(expected));
  }

  #[test]
  fn it_builds_pull_request_comment_at() {
    let repo = Repository { owner: "test_owner".to_owned(), repo_name: "test_repo".to_owned() };
    let comment_id = 1;
    let expected = "https://api.github.com/repos/test_owner/test_repo/pulls/comments/1".to_owned();
    expect!(pull_request_comment_at(&repo, &comment_id)).to(be_equal_to(expected));
  }
}
