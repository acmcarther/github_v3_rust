pub use self::types::{
  PullRequestId,
  PullRequestTitle,
  PullRequestState,
  PullRequestStateQuery,
  PullRequestSortables,
  PullRequestQuery,
  PullRequestUpdate,
  PullRequest,
  CreatePullRequest,
  CreatePullRequestFromIssue,
  MergeRequest,
  MergeFailure,
  MergedResult,
  PullRequestFile,
  PullRequestReference,
  MergedStatus,
  PullRequestComment,
  PullRequestCommentSortable,
  PullRequestCommentQuery,
  CommentId,
  CreateComment,
  ReplyComment,
  EditComment,
  DeleteCommentStatus,
};

mod types {
  use rustc_serialize::{
    Decodable,
    Decoder,
    Encodable,
    Encoder,
  };

  use types::{
    Sha,
    Filename,
    Repository,
    HeadQuery,
    BranchName,
    Message,
    Url,
    GitTm,
    SortDirection,
    IssueId
  };

  use users::types::User;
  use commits::types::Commit;

  pub type PullRequestId = u32;
  pub type PullRequestTitle = String;
  pub type CommentId = u32;

  #[derive(Debug)]
  pub enum PullRequestState {
    Open,
    Closed
  }

  impl Decodable for PullRequestState {
    fn decode<D: Decoder>(d: &mut D) -> Result<PullRequestState, D::Error> {
      d
        .read_str()
        .and_then(|state_str| {
          match state_str.as_ref() {
            "open" => Ok(PullRequestState::Open),
            "closed" => Ok(PullRequestState::Closed),
            _ => {
              let err_str = "no matching pull request state for ".to_owned() + &state_str;
              Err(d.error(&err_str))
            }
          }
        })
    }
  }

  impl Encodable for PullRequestState {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
      let state_str =
        match *self {
          PullRequestState::Open => "open",
          PullRequestState::Closed => "closed"
        };
      s.emit_str(state_str)
    }
  }

  #[derive(Debug)]
  pub enum PullRequestStateQuery {
    Open,
    Closed,
    All
  }

  impl Decodable for PullRequestStateQuery {
    fn decode<D: Decoder>(d: &mut D) -> Result<PullRequestStateQuery, D::Error> {
      d
        .read_str()
        .and_then(|state_str| {
          match state_str.as_ref() {
            "open" => Ok(PullRequestStateQuery::Open),
            "closed" => Ok(PullRequestStateQuery::Closed),
            "all" => Ok(PullRequestStateQuery::All),
            _ => {
              let err_str = "no matching pull request state query for ".to_owned() + &state_str;
              Err(d.error(&err_str))
            }
          }
        })
    }
  }

  impl Encodable for PullRequestStateQuery {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
      let state_str =
        match *self {
          PullRequestStateQuery::Open => "open",
          PullRequestStateQuery::Closed => "closed",
          PullRequestStateQuery::All => "all"
        };
      s.emit_str(state_str)
    }
  }

  #[derive(Debug)]
  pub enum PullRequestSortables {
    CreatedAt,
    UpdatedAt,
    CommentCount,
    LongRunning,
  }

  impl Decodable for PullRequestSortables {
    fn decode<D: Decoder>(d: &mut D) -> Result<PullRequestSortables, D::Error> {
      d
        .read_str()
        .and_then(|state_str| {
          match state_str.as_ref() {
            "created" => Ok(PullRequestSortables::CreatedAt),
            "updated" => Ok(PullRequestSortables::UpdatedAt),
            "popularity" => Ok(PullRequestSortables::CommentCount),
            "long-running" => Ok(PullRequestSortables::LongRunning),
            _ => {
              let err_str = "no matching sort direction for ".to_owned() + &state_str;
              Err(d.error(&err_str))
            }
          }
        })
    }
  }

  impl Encodable for PullRequestSortables {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
      let state_str =
        match *self {
          PullRequestSortables::CreatedAt => "created",
          PullRequestSortables::UpdatedAt => "updated",
          PullRequestSortables::CommentCount => "popularity",
          PullRequestSortables::LongRunning => "long-running",
        };
      s.emit_str(state_str)
    }
  }

  #[derive(RustcEncodable, RustcDecodable, Debug)]
  pub struct PullRequestQuery {
    state: Option<PullRequestStateQuery>,
    head: Option<HeadQuery>,
    base: Option<BranchName>,
    sort: Option<PullRequestSortables>,
    direction: Option<SortDirection>,
  }

  #[derive(RustcEncodable, RustcDecodable, Debug)]
  pub struct PullRequestUpdate {
    pub title: Option<PullRequestTitle>,
    pub body: Option<Message>,
    pub state: Option<PullRequestState>
  }

  #[derive(RustcDecodable, Debug)]
  pub struct PullRequest {
    id: PullRequestId,
    url: Url,
    html_url: Url,
    patch_url: Url,
    issue_url: Url,
    commits_url: Url,
    review_comments_url: Url,
    review_comment_url: Url,
    comments_url: Url,
    number: u32,
    state: PullRequestState,
    title: PullRequestTitle,
    body: Message,
    created_at: GitTm,
    updated_at: GitTm,
    closed_at: Option<GitTm>,
    merged_at: Option<GitTm>,
    user: User,
    head: Commit,
    base: Commit
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
    commit_message: Option<Message>,
    sha: Option<Sha>
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
    sha: Sha,
    filename: Filename,
    status: String,   // TODO: Bound this in an enum
    additions: u32,
    deletions: u32,
    changes: u32,
    blob_url: Url,
    raw_url: Url,
    contents_url: Url,
    patch: String  // TODO: Define this type
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
  pub struct PullRequestComment {
    url: Url,
    id: u32,
    diff_hunk: String,
    path: String,
    position: i32, // TODO: unsigned or signed?
    original_position: i32,
    commit_id: Sha,
    original_commit_id: Sha,
    user: User,
    body: Message,
    created_at: GitTm,
    updated_at: GitTm,
    html_url: Url,
    pull_request_url: Url
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

  impl Decodable for PullRequestCommentSortable {
    fn decode<D: Decoder>(d: &mut D) -> Result<PullRequestCommentSortable, D::Error> {
      d
        .read_str()
        .and_then(|state_str| {
          match state_str.as_ref() {
            "created" => Ok(PullRequestCommentSortable::Created),
            "updated" => Ok(PullRequestCommentSortable::Updated),
            _ => {
              let err_str = "no matching pull request comment sortable for ".to_owned() + &state_str;
              Err(d.error(&err_str))
            }
          }
        })
    }
  }

  impl Encodable for PullRequestCommentSortable {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
      let state_str =
        match *self {
          PullRequestCommentSortable::Created => "created",
          PullRequestCommentSortable::Updated => "updated"
        };
      s.emit_str(state_str)
    }
  }

  #[derive(RustcDecodable, RustcEncodable, Debug)]
  pub struct CreateComment {
    body: Message,
    commit_id: Sha,
    path: String, // TODO: type for this
    position: u32, // TODO: type for this
  }

  #[derive(RustcDecodable, RustcEncodable, Debug)]
  pub struct ReplyComment {
    body: Message,
    in_reply_to: CommentId
  }

  #[derive(RustcDecodable, RustcEncodable, Debug)]
  pub struct EditComment {
    body: Message
  }

  #[allow(dead_code)]
  pub enum DeleteCommentStatus {
    Deleted,
    NotDeleted
  }
}
