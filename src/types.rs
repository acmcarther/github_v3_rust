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
  RequestErr,
  Body,
  GitTm,
  SortDirection,
  Repository,
};

mod types {
  use time::Tm;
  use time::strptime;
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
  pub type RequestErr = String;

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
