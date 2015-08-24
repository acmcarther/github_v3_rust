pub mod types;
pub mod url_builders;

mod pull_requests {
  use hyper::header::Scheme;
  use hyper::method::Method;

  use github_client::GithubClient;

  use std::any::Any;
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
    PullRequestComment,
    PullRequestCommentQuery,
    CommentId,
    CreateComment,
    ReplyComment,
    EditComment,
    DeleteCommentStatus,
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
    fn list_comments(self, pull_request: PullRequestReference) -> Result<Vec<PullRequestComment>, GitErr>;
    fn list_all_pull_request_comments(self, repo: Repository, query: PullRequestCommentQuery) -> Result<Vec<PullRequestComment>, GitErr>;
    fn get_single_comment(self, repo: Repository, comment_id: CommentId) -> Result<PullRequestComment, GitErr>;
    fn create_comment(self, pull_request: PullRequestReference, comment_details: CreateComment) -> Result<PullRequestComment, GitErr>;
    fn create_comment_reply(self, pull_request: PullRequestReference, comment_details: ReplyComment) -> Result<PullRequestComment, GitErr>;
    fn edit_comment(self, repo: Repository, comment_id: CommentId, body: EditComment) -> Result<PullRequestComment, GitErr>;
    #[allow(dead_code, unused_variables)]
    fn delete_comment(self, repo: Repository, comment_id: CommentId) -> Result<DeleteCommentStatus, GitErr>;
  }

  impl<S: Scheme + Any> PullRequester for GithubClient<S> where S::Err: 'static {
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

    fn list_comments(self, pull_request: PullRequestReference) -> Result<Vec<PullRequestComment>, GitErr> {
      let url = url_builders::pull_request_comments(&pull_request.repo, &pull_request.pull_request_id);
      self.request_without_payload(Method::Get, url)
    }

    fn list_all_pull_request_comments(self, repo: Repository, query: PullRequestCommentQuery) -> Result<Vec<PullRequestComment>, GitErr> {
      let url = url_builders::all_pull_request_comments(&repo);
      self.request_with_payload(Method::Patch, url, query)
    }

    fn get_single_comment(self, repo: Repository, comment_id: CommentId) -> Result<PullRequestComment, GitErr> {
      let url = url_builders::pull_request_comment_at(&repo, &comment_id);
      self.request_without_payload(Method::Get, url)
    }

    fn create_comment(self, pull_request: PullRequestReference, comment_details: CreateComment) -> Result<PullRequestComment, GitErr> {
      let url = url_builders::pull_request_comments(&pull_request.repo, &pull_request.pull_request_id);
      self.request_with_payload(Method::Patch, url, comment_details)
    }

    fn create_comment_reply(self, pull_request: PullRequestReference, comment_details: ReplyComment) -> Result<PullRequestComment, GitErr> {
      let url = url_builders::pull_request_comments(&pull_request.repo, &pull_request.pull_request_id);
      self.request_with_payload(Method::Patch, url, comment_details)
    }

    fn edit_comment(self, repo: Repository, comment_id: CommentId, body: EditComment) -> Result<PullRequestComment, GitErr> {
      let url = url_builders::pull_request_comment_at(&repo, &comment_id);
      self.request_with_payload(Method::Patch, url, body)
    }

    #[allow(dead_code, unused_variables)]
    fn delete_comment(self, repo: Repository, comment_id: CommentId) -> Result<DeleteCommentStatus, GitErr> {
      // TODO:
      Err(GitErr::new(ErrorKind::Other, "not implemented".to_owned()))
    }
  }
}
