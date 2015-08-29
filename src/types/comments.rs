use rustc_serialize::{
  Decodable,
  Decoder,
  Encodable,
  Encoder,
};

use types::{
  Url,
  Sha,
  Message,
  GitTm,
  SortDirection,
  Issue,
};

use types::users::GithubUser;
use types::repos::Repo;

use types::pull_requests::PullRequest;

pub type CommentId = u32;

#[derive(RustcDecodable, Debug)]
pub struct PullRequestComment {
  pub url: Url,
  pub id: u32,
  pub diff_hunk: String,
  pub path: String,
  pub position: i32, // TODO: unsigned or signed?
  pub original_position: i32,
  pub commit_id: Sha,
  pub original_commit_id: Sha,
  pub user: GithubUser,
  pub body: Message,
  pub created_at: GitTm,
  pub updated_at: GitTm,
  pub html_url: Url,
  pub pull_request_url: Url
  // TODO: _links
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct PullRequestCommentQuery {
  pub sort: Option<PullRequestCommentSortable>,
  pub direction: Option<SortDirection>,
  pub since: Option<GitTm>
}

#[derive(Debug)]
pub enum PullRequestCommentSortable {
  Created,
  Updated
}

custom_enum_decode_encode!(
  PullRequestCommentSortable [
    "created" <=> [PullRequestCommentSortable::Created],
    "updated" <=> [PullRequestCommentSortable::Updated],
  ]
);

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct CreateCommitComment {
  pub body: Message,
  pub commit_id: Sha,
  pub path: String, // TODO: type for this
  pub position: u32, // TODO: type for this
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct ReplyComment {
  pub body: Message,
  pub in_reply_to: CommentId
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct EditComment {
  pub body: Message
}
#[derive(Debug)]
pub enum IssueCommentEventType {
  Created
}

custom_enum_decode_encode!(
  IssueCommentEventType [
    "created" <=> [IssueCommentEventType::Created],
  ]
);

#[derive(RustcDecodable, Debug)]
pub struct IssueCommentEvent {
  pub action: IssueCommentEventType,
  pub issue: Issue,
  pub comment: IssueComment,
  pub repository: Repo,
  pub sender: GithubUser
}

#[derive(Debug)]
pub enum PullRequestReviewCommentEventType {
  Created
}

custom_enum_decode_encode!(
  PullRequestReviewCommentEventType [
    "created" <=> [PullRequestReviewCommentEventType::Created],
  ]
);

#[derive(RustcDecodable, Debug)]
pub struct PullRequestReviewCommentEvent {
  pub action: PullRequestReviewCommentEventType,
  pub comment: PullRequestComment,
  pub pull_request: PullRequest,
  pub repository: Repo,
  pub sender: GithubUser
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct ListIssueCommentsQuery {
  pub since: GitTm
}

#[derive(Debug)]
pub enum CommentSortables {
  CreatedAt,
  UpdatedAt,
}

custom_enum_decode_encode!(
  CommentSortables [
    "created" <=> [CommentSortables::CreatedAt],
    "updated" <=> [CommentSortables::UpdatedAt],
  ]
);

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct ListRepoCommentsQuery {
  pub sort: CommentSortables,
  pub direction: SortDirection,
  pub since: GitTm
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct CreateIssueComment {
  pub body: Message
}

#[allow(dead_code)]
pub enum DeleteCommentStatus {
  Deleted,
  NotDeleted
}

#[derive(RustcDecodable, Debug)]
pub struct IssueComment {
  pub id: CommentId,
  pub url: Url,
  pub html_url: Url,
  pub body: Message,
  pub user: GithubUser,
  pub created_at: GitTm,
  pub updated_at: GitTm
}
