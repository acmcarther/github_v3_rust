#[macro_export]
macro_rules! custom_enum_encode {
  (
    $enum_ty:ty [ $( $an_enum:pat => $string:expr, )* ]
  ) => {
    impl Encodable for $enum_ty {
      fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        let state_str =
          match *self {
            $($an_enum => $string,)*
          };
        s.emit_str(state_str)
      }
    }
  }
}

#[macro_export]
macro_rules! custom_enum_decode {
  (
    $enum_ty:ty [ $( $string:expr => $an_enum:expr, )* ]
  ) => {
    impl Decodable for $enum_ty {
      fn decode<D: Decoder>(d: &mut D) -> Result<$enum_ty, D::Error> {
        d
          .read_str()
          .and_then(|state_str| {
            match state_str.as_ref() {
              $($string => Ok($an_enum),)*
              _ => {
                let err_str = "no matching item for ".to_owned() + &state_str;
                Err(d.error(&err_str))
              }
            }
          })
      }
    }
  }
}

#[macro_export]
macro_rules! custom_enum_decode_encode {
  (
    $enum_ty:ty [ $($string:tt <=> [$($an_enum:tt)*],)* ]
  ) => {
    custom_enum_decode!($enum_ty [ $( $string => $($an_enum)*, )+ ]);
    custom_enum_encode!($enum_ty [ $( $($an_enum)* => $string, )+ ]);
  }
}

pub mod comments;
pub mod pull_requests;
pub mod repos;
pub mod users;

use time::Tm;
use time::{
  strptime,
  strftime
};
use std::io::Error;
use rustc_serialize::{
  Decodable,
  Decoder,
  Encodable,
  Encoder,
};

use types::repos::{
  Repo,
  Repository,
  LegacyRepo,
};

use std::collections::HashMap;

pub type Body = String;
pub type UserName = String;
pub type HeadQuery = String;
pub type BranchName = String;
pub type IssueId = u32;
pub type Message = String;
pub type Sha = String;
pub type Url = String;
pub type Filename = String;
pub type OrganizationName = String;
pub type GitErr = Error;

#[derive(Debug)]
pub struct GitTm(Tm);

impl Decodable for GitTm {
  fn decode<D: Decoder>(d: &mut D) -> Result<GitTm, D::Error> {
    d
      .read_str()
      .and_then(|time_str| {
        // ISO time
        strptime(&time_str, "%Y-%m-%dT%H:%M:%S%z")
          .map(|time| GitTm(time))
          .map_err(|_| {
            d.error("could not parse time")
          })
      })
  }
}

impl Encodable for GitTm {
  fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
    // TODO: No unwrap?
    let &GitTm(tm) = self;
    let time_str = strftime("%Y-%m-%dT%H:%M:%S%z", &tm).unwrap();
    s.emit_str(&time_str)
  }
}

#[derive(Debug)]
pub enum SortDirection {
  Ascending,
  Descending,
}

custom_enum_decode_encode!(
  SortDirection [
    "asc" <=> [SortDirection::Ascending],
    "desc" <=> [SortDirection::Descending],
  ]
);



#[derive(RustcDecodable, Debug)]
pub struct Commit {
  pub label: BranchName,
  // ref TODO: custom decoder for reserved word
  pub sha: Sha,
  pub user: GithubUser,
  pub repo: Repo,
}

#[derive(RustcDecodable, Debug)]
pub struct GithubCommit {
  pub url: Url,
  pub sha: Sha,
  pub html_url: Url,
  pub comments_url: Url,
  pub commit: GithubCommitSummary,
  pub author: GithubUser,
  pub committer: GithubUser,
  pub parents: Vec<CommitTreeNode>
}

#[derive(RustcDecodable, Debug)]
pub struct GithubCommitSummary {
  pub url: Url,
  pub author: CommitAuthor,
  pub committer: CommitAuthor,
  pub message: Message,
  pub tree: CommitTreeNode,
  pub comment_count: u32,
}

#[derive(RustcDecodable, Debug)]
pub struct CommitAuthor {
  pub name: String,
  pub email: String,
  pub date: GitTm
}

#[derive(RustcDecodable, Debug)]
pub struct CommitTreeNode {
  pub url: Url,
  pub sha: Sha,
}

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

#[derive(RustcDecodable, Debug)]
pub struct PushEvent {
  // TODO: custom decode for key ref
  pub before: Sha,
  pub after: Sha,
  pub created: bool,
  pub deleted: bool,
  pub forced: bool,
  pub base_ref: Option<Sha>,
  pub compare: Url,
  pub commits: Vec<PushCommit>,
  pub head_commit: PushCommit,
  pub repository: LegacyRepo,
  pub pusher: GitUser,
  pub sender: GithubUser,
}

#[derive(RustcDecodable, Debug)]
pub struct PushCommit {
  pub id: Sha,
  pub distinct: bool,
  pub message: Message,
  pub timestamp: GitTm,
  pub url: Url,
  pub author: GitUser,
  pub committer: GitUser,
  pub added: Vec<Filename>,
  pub removed: Vec<Filename>,
  pub modified: Vec<Filename>,
}


#[derive(RustcDecodable, Debug)]
pub struct Issue {
  pub url: Url,
  pub labels_url: Url,
  pub comments_url: Url,
  pub events_url: Url,
  pub html_url: Url,
  pub id: IssueId,
  pub number: u32,
  pub title: Message,
  pub user: GithubUser
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

#[allow(dead_code)]
pub struct MergeRequest {
  pub commit_message: Option<Message>,
  pub sha: Option<Sha>
}

#[allow(dead_code)]
pub enum MergeFailure {
  NotPossible,
  ShaDidNotMatch,
}

#[allow(dead_code)]
pub enum MergedResult {
  Success { sha: Sha, message: Message },
  Failure { failure_type: MergeFailure, message: Message, documentation_url: Url }
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

#[allow(dead_code)]
pub enum MergedStatus {
  Merged,
  NotMerged
}

#[derive(RustcDecodable, Debug)]
pub struct Organization {
  pub login: UserName,
  pub id: u32,
  pub avatar_url: Url,
  pub gravatar_id: String,
  pub url: Url,
  pub html_url: Url,
  pub followers_url: Url,
  pub following_url: Url,
  pub gists_url: Url,
  pub subscriptions_url: Url,
  pub organizations_url: Url,
  pub repos_url: Url,
  pub events_url: Url,
  pub received_events_url: Url,
  // type: String   TODO: Custom decode for this key
  pub site_admin: bool
}

#[derive(RustcEncodable, Debug)]
pub struct ContributorsQuery {
  pub anon: bool
}

#[derive(RustcDecodable, Debug)]
pub struct LanguagePile(HashMap<String, u32>); // TODO: Types

// TODO: Types
#[derive(RustcDecodable, Debug)]
pub struct Team {
  pub id: u32,
  pub url: Url,
  pub name: String,
  pub slug: String,
  pub description: String,
  pub privacy: String,
  pub permission: String,
  pub members_url: Url,
  pub repositories_url: Url,
}

// TODO: Types
#[derive(RustcDecodable, Debug)]
pub struct Tag {
  pub name: String,
  pub commit: CommitTreeNode,
  pub zipball_url: Url,
  pub tarball_url: Url
}

#[derive(RustcDecodable, Debug)]
pub struct Branch {
  pub name: BranchName,
  pub commit: CommitTreeNode,
}

#[derive(RustcDecodable, Debug)]
pub struct FullBranch {
  pub name: BranchName,
  pub commit: GithubCommit
  // TODO: _links
}

#[allow(dead_code)]
pub enum DeletedStatus {
  Deleted,
  NotDeleted
}


#[derive(RustcDecodable, Debug)]
pub struct GithubUser {
  pub login: UserName,
  pub id: u32,
  pub avatar_url: Url,
  pub gravatar_id: String,  // TODO: What is this
  pub html_url: Url,
  pub followers_url: Url,
  pub following_url: Url,
  pub gists_url: Url,
  pub starred_url: Url,
  pub subscriptions_url: Url,
  pub organizations_url: Url,
  pub repos_url: Url,
  pub events_url: Url,
  pub received_events_url: Url,
  //type: String   // TODO: Custom decode for this reserved word
  pub site_admin: bool
}

#[derive(RustcDecodable, Debug)]
pub struct GitUser {
  pub name: String,
  pub email: String,
  pub username: Option<UserName>
}
