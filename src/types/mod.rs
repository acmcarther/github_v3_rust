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
pub mod commits;
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

use types::repos::LegacyRepo;

use types::commits::{
  PushCommit,
  CommitTreeNode,
  GithubCommit
};

use types::users::{
  GitUser,
  GithubUser,
  UserName
};

use std::collections::HashMap;

pub type Body = String;
pub type HeadQuery = String;
pub type BranchName = String;
pub type IssueId = u32;
pub type Message = String;
pub type Sha = String;
pub type Url = String;
pub type Filename = String;
pub type OrganizationName = String;
pub type GitErr = Error;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[allow(dead_code)]
#[derive(Debug)]
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

#[derive(RustcEncodable, Debug, PartialEq)]
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
