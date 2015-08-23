pub use self::url_builders::{
  pull_requests,
  pull_request_at,
  pull_request_commits,
  pull_request_files,
  pull_request_merge
};

mod url_builders {
  use types::{
    Repository,
    Url,
  };

  use pull_requests::types::{
    PullRequestId
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

  pub fn pull_request_merge(repo: &Repository, pr_id: &PullRequestId) -> Url {
    pull_request_at(repo, pr_id) + "/merge"
  }

  #[cfg(test)]
  mod tests {
    pub use types::{
      Repository,
      Url,
    };

    pub use super::{
      pull_requests,
      pull_request_at,
      pull_request_commits,
      pull_request_files,
      pull_request_merge
    };

    describe! url_builders {
      before_each {
        let repo = Repository { owner: "test_owner".to_owned(), repo_name: "test_repo".to_owned() };
        let pr_id = 21;
        let repo_url = "https://api.github.com/repos/test_owner/test_repo".to_owned();
      }

      it "builds 'pull_requests'" {
        let expected = repo_url + "/pulls";
        assert_eq!(pull_requests(&repo), expected);
      }

      it "builds 'pull_request_at'" {
        let expected = repo_url + "/pulls/21";
        assert_eq!(pull_request_at(&repo, &pr_id), expected);
      }

      it "builds 'pull_request_commits'" {
        let expected = repo_url + "/pulls/21/commits";
        assert_eq!(pull_request_commits(&repo, &pr_id), expected);
      }

      it "builds 'pull_request_files'" {
        let expected = repo_url + "/pulls/21/files";
        assert_eq!(pull_request_files(&repo, &pr_id), expected);
      }

      it "builds 'pull_request_merge'" {
        let expected = repo_url + "/pulls/21/merge";
        assert_eq!(pull_request_merge(&repo, &pr_id), expected);
      }
    }
  }
}
