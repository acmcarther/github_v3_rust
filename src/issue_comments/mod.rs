pub mod url_builders;

use hyper::method::Method;

use github_client::{SimpleClient};

use std::io::ErrorKind;

use types::{
  GitErr,
  IssueId,
  CommentId,
  ListIssueCommentsQuery,
  ListRepoCommentsQuery,
  CreateIssueComment,
  EditComment,
  DeleteCommentStatus,
  IssueComment,
};

use types::repos::Repository;

pub trait IssueCommenter {
  fn list_in_issue(&self, repo: Repository, issue_id: IssueId, query: Option<ListIssueCommentsQuery>) -> Result<Vec<IssueComment>, GitErr>;
  fn list_in_repo(&self, repo: Repository, query: Option<ListRepoCommentsQuery>) -> Result<Vec<IssueComment>, GitErr>;
  fn get_comment(&self, repo: Repository, comment_id: CommentId) -> Result<IssueComment, GitErr>;
  fn create_comment(&self, repo: Repository, issue_id: IssueId, details: CreateIssueComment) -> Result<IssueComment, GitErr>;
  fn edit_comment(&self, repo: Repository, comment_id: CommentId, details: EditComment) -> Result<IssueComment, GitErr>;
  fn delete_comment(&self, repo: Repository, comment_id: CommentId) -> Result<DeleteCommentStatus, GitErr>;
}

impl<C: SimpleClient> IssueCommenter for C {
  fn list_in_issue(&self, repo: Repository, issue_id: IssueId, query: Option<ListIssueCommentsQuery>) -> Result<Vec<IssueComment>, GitErr> {
    let url = url_builders::issue_comments(&repo, &issue_id);
    match query {
      Some(query) => self.request_with_payload(Method::Get, url, query),
      None => self.request_without_payload(Method::Get, url)
    }
  }

  fn list_in_repo(&self, repo: Repository, query: Option<ListRepoCommentsQuery>) -> Result<Vec<IssueComment>, GitErr> {
    let url = url_builders::issue_comments_for_repo(&repo);
    match query {
      Some(query) => self.request_with_payload(Method::Get, url, query),
      None => self.request_without_payload(Method::Get, url)
    }
  }

  fn get_comment(&self, repo: Repository, comment_id: CommentId) -> Result<IssueComment, GitErr> {
    let url = url_builders::issue_comment_at(&repo, &comment_id);
    self.request_without_payload(Method::Get, url)
  }

  fn create_comment(&self, repo: Repository, issue_id: IssueId, details: CreateIssueComment) -> Result<IssueComment, GitErr> {
    let url = url_builders::issue_comments(&repo, &issue_id);
    self.request_with_payload(Method::Post, url, details)
  }

  fn edit_comment(&self, repo: Repository, comment_id: CommentId, details: EditComment) -> Result<IssueComment, GitErr> {
    let url = url_builders::issue_comment_at(&repo, &comment_id);
    self.request_with_payload(Method::Post, url, details)
  }

  #[allow(unused_variables)]
  fn delete_comment(&self, repo: Repository, comment_id: CommentId) -> Result<DeleteCommentStatus, GitErr> {
    // TODO:
    Err(GitErr::new(ErrorKind::Other, "not implemented".to_owned()))
  }
}
