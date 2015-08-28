pub mod types;
pub mod url_builders;

use hyper::method::Method;

use github_client::{SimpleClient};

use std::io::ErrorKind;

use types::{
  GitErr,
  Repository,
};

use pull_requests::types::{
  PullRequestReference,
};

use commit_comments::types::{
  PullRequestComment,
  PullRequestCommentQuery,
  CommentId,
  CreateComment,
  ReplyComment,
  EditComment,
  DeleteCommentStatus,
};

pub trait CommitCommenter {
  fn list_comments(&self, pull_request: PullRequestReference) -> Result<Vec<PullRequestComment>, GitErr>;
  fn list_all_pull_request_comments(&self, repo: Repository, query: PullRequestCommentQuery) -> Result<Vec<PullRequestComment>, GitErr>;
  fn get_single_comment(&self, repo: Repository, comment_id: CommentId) -> Result<PullRequestComment, GitErr>;
  fn create_comment(&self, pull_request: PullRequestReference, comment_details: CreateComment) -> Result<PullRequestComment, GitErr>;
  fn create_comment_reply(&self, pull_request: PullRequestReference, comment_details: ReplyComment) -> Result<PullRequestComment, GitErr>;
  fn edit_comment(&self, repo: Repository, comment_id: CommentId, body: EditComment) -> Result<PullRequestComment, GitErr>;
  #[allow(dead_code, unused_variables)]
  fn delete_comment(&self, repo: Repository, comment_id: CommentId) -> Result<DeleteCommentStatus, GitErr>;
}

impl<C: SimpleClient> CommitCommenter for C {
  fn list_comments(&self, pull_request: PullRequestReference) -> Result<Vec<PullRequestComment>, GitErr> {
    let url = url_builders::pull_request_comments(&pull_request.repo, &pull_request.pull_request_id);
    self.request_without_payload(Method::Get, url)
  }

  fn list_all_pull_request_comments(&self, repo: Repository, query: PullRequestCommentQuery) -> Result<Vec<PullRequestComment>, GitErr> {
    let url = url_builders::all_pull_request_comments(&repo);
    self.request_with_payload(Method::Patch, url, query)
  }

  fn get_single_comment(&self, repo: Repository, comment_id: CommentId) -> Result<PullRequestComment, GitErr> {
    let url = url_builders::pull_request_comment_at(&repo, &comment_id);
    self.request_without_payload(Method::Get, url)
  }

  fn create_comment(&self, pull_request: PullRequestReference, comment_details: CreateComment) -> Result<PullRequestComment, GitErr> {
    let url = url_builders::pull_request_comments(&pull_request.repo, &pull_request.pull_request_id);
    self.request_with_payload(Method::Patch, url, comment_details)
  }

  fn create_comment_reply(&self, pull_request: PullRequestReference, comment_details: ReplyComment) -> Result<PullRequestComment, GitErr> {
    let url = url_builders::pull_request_comments(&pull_request.repo, &pull_request.pull_request_id);
    self.request_with_payload(Method::Patch, url, comment_details)
  }

  fn edit_comment(&self, repo: Repository, comment_id: CommentId, body: EditComment) -> Result<PullRequestComment, GitErr> {
    let url = url_builders::pull_request_comment_at(&repo, &comment_id);
    self.request_with_payload(Method::Patch, url, body)
  }

  #[allow(dead_code, unused_variables)]
  fn delete_comment(&self, repo: Repository, comment_id: CommentId) -> Result<DeleteCommentStatus, GitErr> {
    // TODO:
    Err(GitErr::new(ErrorKind::Other, "not implemented".to_owned()))
  }
}
