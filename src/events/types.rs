pub use self::types::{
  IssueCommentEvent,
  IssueCommentEventType,
  PullRequestEvent,
  PullRequestEventType,
  PullRequestReviewCommentEventType,
  PullRequestReviewCommentEvent,
  PushEvent,
  Pusher,
  PushCommit,
};

mod types {
  use rustc_serialize::{
    Decoder,
    Decodable,
    Encoder,
    Encodable,
  };

  use types::{
    Url,
    Sha,
    UserName,
    GitTm,
    Filename,
    Message
  };

  use users::types::User;
  use repos::types::Repo;
  use issue_comments::types::{
    IssueComment,
    Issue
  };

  use pull_requests::types::{
    PullRequest
  };

  use commit_comments::types::{
    PullRequestComment,
  };

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
    pub sender: User
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
    pub sender: User
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
    pub sender: User
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
    pub repository: Repo,
    pub pusher: Pusher,
    pub sender: User,
  }

  #[derive(RustcDecodable, Debug)]
  pub struct Pusher {
    pub name: String,
    pub email: String,
    pub username: Option<UserName>
  }

  #[derive(RustcDecodable, Debug)]
  pub struct PushCommit {
    pub id: Sha,
    pub distinct: bool,
    pub message: Message,
    pub timestamp: GitTm,
    pub url: Url,
    pub author: Pusher,
    pub committer: Pusher,
    pub added: Vec<Filename>,
    pub removed: Vec<Filename>,
    pub modified: Vec<Filename>,
  }
}
