pub use self::url_builders::{
  pull_requests,
  pull_request_at,
  pull_request_commits,
  pull_request_files,
  pull_request_merge,
  pull_request_comments,
  all_pull_request_comments,
  pull_request_comment_at,
};

mod url_builders {
  use types::{
    Repository,
    Url,
  };

  use pull_requests::types::{
    PullRequestId,
    CommentId
  };

  pub fn pull_requests(repo: &Repository) -> Url {
    "https://api.github.com/repos/".to_owned() +
      &repo.owner + "/" +
      &repo.repo_name + "/pulls"
  }

  pub fn pull_request_at(repo: &Repository, pr_id: &PullRequestId) -> Url {
    pull_requests(repo) + "/" + &pr_id.to_string()
  }

  pub fn pull_request_commits(repo: &Repository, pr_id: &PullRequestId) -> Url {
    pull_request_at(repo, pr_id) + "/commits"
  }

  pub fn pull_request_files(repo: &Repository, pr_id: &PullRequestId) -> Url {
    pull_request_at(repo, pr_id) + "/files"
  }

  #[allow(dead_code)]
  pub fn pull_request_merge(repo: &Repository, pr_id: &PullRequestId) -> Url {
    pull_request_at(repo, pr_id) + "/merge"
  }

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
      Repository,
      Url,
    };

    pub use super::{
      pull_requests,
      pull_request_at,
      pull_request_commits,
      pull_request_files,
      pull_request_merge,
      pull_request_comments,
      all_pull_request_comments,
      pull_request_comment_at,
    };

    #[test]
    fn it_builds_pull_requests() {
      let repo = Repository { owner: "test_owner".to_owned(), repo_name: "test_repo".to_owned() };
      let expected = "https://api.github.com/repos/test_owner/test_repo/pulls".to_owned();
      expect!(pull_requests(&repo)).to(be_equal_to(expected));
    }

    #[test]
    fn it_builds_pull_request_at() {
      let repo = Repository { owner: "test_owner".to_owned(), repo_name: "test_repo".to_owned() };
      let pr_id = 21;
      let expected = "https://api.github.com/repos/test_owner/test_repo/pulls/21".to_owned();
      expect!(pull_request_at(&repo, &pr_id)).to(be_equal_to(expected));
    }

    #[test]
    fn it_builds_pull_request_commits() {
      let repo = Repository { owner: "test_owner".to_owned(), repo_name: "test_repo".to_owned() };
      let pr_id = 21;
      let expected = "https://api.github.com/repos/test_owner/test_repo/pulls/21/commits".to_owned();
      expect!(pull_request_commits(&repo, &pr_id)).to(be_equal_to(expected));
    }

    #[test]
    fn it_builds_pull_request_files() {
      let repo = Repository { owner: "test_owner".to_owned(), repo_name: "test_repo".to_owned() };
      let pr_id = 21;
      let expected = "https://api.github.com/repos/test_owner/test_repo/pulls/21/files".to_owned();
      expect!(pull_request_files(&repo, &pr_id)).to(be_equal_to(expected));
    }

    #[test]
    fn it_builds_pull_request_merge() {
      let repo = Repository { owner: "test_owner".to_owned(), repo_name: "test_repo".to_owned() };
      let pr_id = 21;
      let expected = "https://api.github.com/repos/test_owner/test_repo/pulls/21/merge".to_owned();
      expect!(pull_request_merge(&repo, &pr_id)).to(be_equal_to(expected));
    }

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
}
