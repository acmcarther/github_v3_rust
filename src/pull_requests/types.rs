pub use self::types::{
  PullRequestId,
  PullRequestTitle,
  PullRequestState,
  PullRequestStateQuery,
  PullRequestSortables,
  PullRequestQuery,
  PullRequestUpdate,
  PullRequest,
  MergeRequest,
  MergeFailure,
  MergedResult,
  UpdatedPullRequest,
  PullRequestFile,
  PullRequestReference,
  MergedStatus
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
  };

  use users::types::User;
  use commits::types::Commit;

  pub type PullRequestId = u32;
  pub type PullRequestTitle = String;

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
              let err_str = "no matching pull request state for {}".to_owned() + &state_str;
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
              let err_str = "no matching pull request state query for {}".to_owned() + &state_str;
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
              let err_str = "no matching sort direction for {}".to_owned() + &state_str;
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

  #[derive(RustcDecodable, Debug)]
  pub struct PullRequestQuery {
    state: Option<PullRequestStateQuery>,
    head: Option<HeadQuery>,
    base: Option<BranchName>,
    sort: Option<PullRequestSortables>,
    direction: Option<SortDirection>,
  }

  pub struct PullRequestUpdate {
    title: Option<PullRequestTitle>,
    body: Option<Message>,
    state: Option<PullRequestState>
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

  pub struct MergeRequest {
    commit_message: Option<Message>,
    sha: Option<Sha>
  }

  pub enum MergeFailure {
    NotPossible,
    ShaDidNotMatch,
  }

  pub enum MergedResult {
    Success { sha: Sha, message: Message },
    Failure { failure_type: MergeFailure, message: Message, documentation_url: Url }
  }

  // TODO: Build this out with all of the data
  pub struct UpdatedPullRequest {
    id: PullRequestId
  }

  // TODO: Build this out with all of the data
  pub struct PullRequestFile {
    sha: Sha,
    filename: Filename,
  }

  pub struct PullRequestReference {
    repo: Repository,
    pull_request_id: PullRequestId
  }

  pub enum MergedStatus {
    Merged,
    NotMerged
  }

}



