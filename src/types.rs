pub use self::types::{
  UserName,
  RepoName,
  HeadQuery,
  BranchName,
  IssueId,
  Message,
  Sha,
  Url,
  Filename,
  GitErr,
  Body,
  GitTm,
  SortDirection,
  Repository,
  OrganizationName,
};

mod types {
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

  pub type Body = String;
  pub type UserName = String;
  pub type HeadQuery = String;
  pub type RepoName = String;
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

  impl Decodable for SortDirection {
    fn decode<D: Decoder>(d: &mut D) -> Result<SortDirection, D::Error> {
      d
        .read_str()
        .and_then(|state_str| {
          match state_str.as_ref() {
            "asc" => Ok(SortDirection::Ascending),
            "desc" => Ok(SortDirection::Descending),
            _ => {
              let err_str = "no matching sort direction for {}".to_owned() + &state_str;
              Err(d.error(&err_str))
            }
          }
        })
    }
  }

  impl Encodable for SortDirection {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
      let state_str =
        match *self {
          SortDirection::Ascending => "asc",
          SortDirection::Descending => "desc",
        };
      s.emit_str(state_str)
    }
  }

  pub struct Repository {
    pub owner: UserName,
    pub repo_name: RepoName
  }
}
