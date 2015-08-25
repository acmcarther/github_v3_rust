pub use self::pull_requests::{
  PullRequester
};

pub mod types;
pub mod url_builders;

mod pull_requests {
  use hyper::method::Method;

  use github_client::{SimpleClient};

  use std::io::ErrorKind;

  use types::{
    GitErr,
    Repository,
  };

  use pull_requests::types::{
    PullRequestId,
    PullRequestQuery,
    PullRequestUpdate,
    PullRequest,
    CreatePullRequest,
    CreatePullRequestFromIssue,
    MergeRequest,
    MergedResult,
    PullRequestFile,
    PullRequestReference,
    MergedStatus,
  };

  use pull_requests::url_builders;

  use commits::types::GithubCommit;

  pub trait PullRequester {
    fn list(self, repo: Repository, query: Option<PullRequestQuery>) -> Result<Vec<PullRequest>, GitErr>;
    fn get_pr(self, repo: Repository, pr_id: PullRequestId) -> Result<PullRequest, GitErr>;
    fn create_raw(self, repo: Repository, details: CreatePullRequest) -> Result<PullRequest, GitErr>;
    fn create_from_issue(self, repo: Repository, details: CreatePullRequestFromIssue) -> Result<PullRequest, GitErr>;
    fn update_pull_request(self, pull_request: PullRequestReference, update: PullRequestUpdate) -> Result<PullRequest, GitErr>;
    fn list_commits(self, pull_request: PullRequestReference) -> Result<Vec<GithubCommit>, GitErr>;
    fn list_files(self, pull_request: PullRequestReference) -> Result<Vec<PullRequestFile>, GitErr>;
    #[allow(dead_code, unused_variables)]
    fn get_merged(self, pull_request: PullRequestReference) -> Result<MergedStatus, GitErr>;
    #[allow(dead_code, unused_variables)]
    fn merge(self, pull_request: PullRequestReference, merge_request: Option<MergeRequest>) -> Result<MergedResult, GitErr>;
  }

  impl<C: SimpleClient> PullRequester for C {
    fn list(self, repo: Repository, query: Option<PullRequestQuery>) -> Result<Vec<PullRequest>, GitErr> {
      let url = url_builders::pull_requests(&repo);
      match query {
        Some(query) => self.request_with_payload(Method::Post, url, query),
        None => self.request_without_payload(Method::Get, url)
      }
    }

    fn get_pr(self, repo: Repository, pr_id: PullRequestId) -> Result<PullRequest, GitErr> {
      let url = url_builders::pull_request_at(&repo, &pr_id);
      self.request_without_payload(Method::Get, url)
    }

    fn create_raw(self, repo: Repository, details: CreatePullRequest) -> Result<PullRequest, GitErr> {
      let url = url_builders::pull_requests(&repo);
      self.request_with_payload(Method::Post, url, details)
    }

    fn create_from_issue(self, repo: Repository, details: CreatePullRequestFromIssue) -> Result<PullRequest, GitErr> {
      let url = url_builders::pull_requests(&repo);
      self.request_with_payload(Method::Post, url, details)
    }

    fn update_pull_request(self, pull_request: PullRequestReference, update: PullRequestUpdate) -> Result<PullRequest, GitErr> {
      let url = url_builders::pull_request_at(&pull_request.repo, &pull_request.pull_request_id);
      self.request_with_payload(Method::Patch, url, update)
    }

    fn list_commits(self, pull_request: PullRequestReference) -> Result<Vec<GithubCommit>, GitErr> {
      let url = url_builders::pull_request_commits(&pull_request.repo, &pull_request.pull_request_id);
      self.request_without_payload(Method::Get, url)
    }

    fn list_files(self, pull_request: PullRequestReference) -> Result<Vec<PullRequestFile>, GitErr> {
      let url = url_builders::pull_request_files(&pull_request.repo, &pull_request.pull_request_id);
      self.request_without_payload(Method::Get, url)
    }

    #[allow(dead_code, unused_variables)]
    fn get_merged(self, pull_request: PullRequestReference) -> Result<MergedStatus, GitErr> {
      // TODO:
      Err(GitErr::new(ErrorKind::Other, "not implemented".to_owned()))
    }

    #[allow(dead_code, unused_variables)]
    fn merge(self, pull_request: PullRequestReference, merge_request: Option<MergeRequest>) -> Result<MergedResult, GitErr> {
      // TODO:
      Err(GitErr::new(ErrorKind::Other, "not implemented".to_owned()))
    }
  }
}
