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
    pub url: Url,
    pub id: u32,
    pub diff_hunk: String,
    pub path: String,
    pub position: i32, // TODO: unsigned or signed?
    pub original_position: i32,
    pub commit_id: Sha,
    pub original_commit_id: Sha,
    pub user: User,
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
  pub struct CreateComment {
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

  #[allow(dead_code)]
  pub enum DeleteCommentStatus {
    Deleted,
    NotDeleted
  }
}
