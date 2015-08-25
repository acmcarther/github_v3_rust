pub use self::types::{
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
    Message,
    Url,
    GitTm,
    SortDirection,
  };

  use users::types::User;

  pub type CommentId = u32;

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

  custom_enum_decode_encode!(
    PullRequestCommentSortable [
      "created" <=> [PullRequestCommentSortable::Created],
      "updated" <=> [PullRequestCommentSortable::Updated],
    ]
  );

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
