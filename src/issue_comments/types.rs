pub use self::types::{
  CommentId
};

mod types {
  use rustc_serialize::{
    Decodable,
    Decoder,
    Encodable,
    Encoder,
  };

  use types::{
    Message,
    Url,
    GitTm,
    SortDirection,
    IssueId
  };

  use users::types::User;

  pub type CommentId = u32;

  #[derive(RustcEncodable, RustcDecodable, Debug)]
  pub struct ListIssueCommentsQuery {
    since: GitTm
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
    sort: CommentSortables,
    direction: SortDirection,
    since: GitTm
  }

  #[derive(RustcEncodable, RustcDecodable, Debug)]
  pub struct NewComment {
    body: Message
  }

  pub type EditComment = NewComment;

  #[allow(dead_code)]
  pub enum DeleteCommentStatus {
    Deleted,
    NotDeleted
  }

  #[derive(RustcDecodable, Debug)]
  pub struct IssueComment {
    id: CommentId,
    url: Url,
    html_url: Url,
    body: Message,
    user: User,
    created_at: GitTm,
    updated_at: GitTm
  }

}
