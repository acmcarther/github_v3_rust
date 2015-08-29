use rustc_serialize::{
  Decodable,
  Decoder,
  Encodable,
  Encoder,
};

use types::{
  Url,
  GithubUser,
  HeadQuery,
  Filename,
  BranchName,
  Message,
  SortDirection,
  GitTm,
  IssueId,
  Sha
};

use types::repos::{
  Repo,
  Repository,
};

use types::commits::{
  Commit
};

#[derive(Debug)]
pub enum PullRequestEventType {
  Assigned,
  Unassigned,
  Labeled,
  Unlabeled,
  Opened,
  Closed,
  Synchronize
}

custom_enum_decode_encode!(
  PullRequestEventType [
    "assigned" <=> [PullRequestEventType::Assigned],
    "unassigned" <=> [PullRequestEventType::Unassigned],
    "labeled" <=> [PullRequestEventType::Labeled],
    "unlabeled" <=> [PullRequestEventType::Unlabeled],
    "opened" <=> [PullRequestEventType::Opened],
    "closed" <=> [PullRequestEventType::Closed],
    "synchronize" <=> [PullRequestEventType::Synchronize],
  ]
);


#[derive(RustcDecodable, Debug)]
pub struct PullRequestEvent {
  pub action: PullRequestEventType,
  pub number: u32,
  pub pull_request: PullRequest,
  pub repository: Repo,
  pub sender: GithubUser
}

pub type PullRequestId = u32;
pub type PullRequestTitle = String;

#[derive(Debug)]
pub enum PullRequestState {
  Open,
  Closed
}

custom_enum_decode_encode!(
  PullRequestState [
    "open" <=> [PullRequestState::Open],
    "closed" <=> [PullRequestState::Closed],
  ]
);

#[derive(Debug)]
pub enum PullRequestStateQuery {
  Open,
  Closed,
  All
}

custom_enum_decode_encode!(
  PullRequestStateQuery [
    "open" <=> [PullRequestStateQuery::Open],
    "closed" <=> [PullRequestStateQuery::Closed],
    "all" <=> [PullRequestStateQuery::All],
  ]
);

#[derive(Debug)]
pub enum PullRequestSortables {
  CreatedAt,
  UpdatedAt,
  CommentCount,
  LongRunning,
}

custom_enum_decode_encode!(
  PullRequestSortables [
    "created" <=> [PullRequestSortables::CreatedAt],
    "updated" <=> [PullRequestSortables::UpdatedAt],
    "popularity" <=> [PullRequestSortables::CommentCount],
    "long-running" <=> [PullRequestSortables::LongRunning],
  ]
);

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct PullRequestQuery {
  pub state: Option<PullRequestStateQuery>,
  pub head: Option<HeadQuery>,
  pub base: Option<BranchName>,
  pub sort: Option<PullRequestSortables>,
  pub direction: Option<SortDirection>,
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct PullRequestUpdate {
  pub title: Option<PullRequestTitle>,
  pub body: Option<Message>,
  pub state: Option<PullRequestState>
}

#[derive(RustcDecodable, Debug)]
pub struct PullRequest {
  pub id: PullRequestId,
  pub url: Url,
  pub html_url: Url,
  pub patch_url: Url,
  pub issue_url: Url,
  pub commits_url: Url,
  pub review_comments_url: Url,
  pub review_comment_url: Url,
  pub comments_url: Url,
  pub number: u32,
  pub state: PullRequestState,
  pub title: PullRequestTitle,
  pub body: Message,
  pub created_at: GitTm,
  pub updated_at: GitTm,
  pub closed_at: Option<GitTm>,
  pub merged_at: Option<GitTm>,
  pub user: GithubUser,
  pub head: Commit,
  pub base: Commit
  // TODO: _links
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct CreatePullRequest {
  pub title: PullRequestTitle,
  pub head: HeadQuery,
  pub base: BranchName,
  pub body: Option<Message>
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct CreatePullRequestFromIssue {
  pub head: HeadQuery,
  pub base: BranchName,
  pub issue: IssueId
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct PullRequestFile {
  pub sha: Sha,
  pub filename: Filename,
  pub status: String,   // TODO: Bound this in an enum
  pub additions: u32,
  pub deletions: u32,
  pub changes: u32,
  pub blob_url: Url,
  pub raw_url: Url,
  pub contents_url: Url,
  pub patch: String  // TODO: Define this type
}

pub struct PullRequestReference {
  pub repo: Repository,
  pub pull_request_id: PullRequestId
}

