pub mod types;
pub mod url_builders;

mod pull_requests {
  use hyper::header::Scheme;
  use hyper::client::response::Response;
  use rustc_serialize::Decodable;
  use rustc_serialize::json::DecoderError;

  use github_client::GithubClient;

  use std::any::Any;
  use std::io::Read;
  use std::io::ErrorKind;
  use std::error::Error as StdError;

  use rustc_serialize::{
    json
  };

  use types::{
    RequestErr,
    Repository,
  };

  use pull_requests::types::{
    PullRequestId,
    PullRequestQuery,
    PullRequestUpdate,
    PullRequest,
    CreatePullRequest,
    CreatePullRequestFromIssue,
    MergeRequest,
    MergedResult,
    PullRequestFile,
    PullRequestReference,
    MergedStatus
  };

  use pull_requests::url_builders;

  use commits::types::Commit;

  fn deserialize<S: Decodable>(response: Response) -> Result<S, DecoderError> {
    let mut response = response;
    let mut buf = String::new();
    let _ = response.read_to_string(&mut buf);
    println!("recv: {}", buf);
    json::decode(&buf)
  }

  pub trait PullRequester {
    fn list(self, repo: Repository, query: Option<PullRequestQuery>) -> Result<Vec<PullRequest>, RequestErr>;
    fn get_pr(self, repo: Repository, pr_id: PullRequestId) -> Result<PullRequest, RequestErr>;
    fn create_raw(self, repo: Repository, details: CreatePullRequest) -> Result<PullRequest, RequestErr>;
    fn create_from_issue(self, repo: Repository, details: CreatePullRequestFromIssue) -> Result<PullRequest, RequestErr>;
    fn update_pull_request(self, pull_request: PullRequestReference, update: PullRequestUpdate) -> Result<PullRequest, RequestErr>;
    fn list_commits(self, pull_request: PullRequestReference) -> Result<Vec<Commit>, RequestErr>;
    fn list_files(self, pull_request: PullRequestReference) -> Result<Vec<PullRequestFile>, RequestErr>;
    fn get_merged(self, pull_request: PullRequestReference) -> Result<MergedStatus, RequestErr>;
    fn merge(self, pull_request: PullRequestReference, merge_request: Option<MergeRequest>) -> Result<MergedResult, RequestErr>;
  }

  impl<S: Scheme + Any> PullRequester for GithubClient<S> where S::Err: 'static {
    fn list(self, repo: Repository, query: Option<PullRequestQuery>) -> Result<Vec<PullRequest>, RequestErr> {
      let url = url_builders::pull_requests(&repo);
      let query_body = query.map(|query| json::encode(&query));
      match query_body {
        Some(query_res) => {
          query_res
            .map_err(|err| RequestErr::new(ErrorKind::Other, err.description().to_owned()))
            .and_then(|query| {
              self.get(url, Some(query))
                .map_err(|_| RequestErr::new(ErrorKind::Other, "Request failed".to_owned()))
                .and_then(|res| deserialize(res).map_err(|err| RequestErr::new(ErrorKind::Other, err.description().to_owned())))
            })
        },
        None => {
          self.get(url, None)
            .map_err(|_| RequestErr::new(ErrorKind::Other, "Request failed".to_owned()))
            .and_then(|res| deserialize(res).map_err(|err| RequestErr::new(ErrorKind::Other, err.description().to_owned())))
        }
      }
    }

    fn get_pr(self, repo: Repository, pr_id: PullRequestId) -> Result<PullRequest, RequestErr> {
      let url = url_builders::pull_request_at(&repo, &pr_id);
      self.get(url, None)
        .map_err(|_| RequestErr::new(ErrorKind::Other, "Request failed".to_owned()))
        .and_then(|res| deserialize(res).map_err(|err| RequestErr::new(ErrorKind::Other, err.description().to_owned())))
    }

    fn create_raw(self, repo: Repository, details: CreatePullRequest) -> Result<PullRequest, RequestErr> {
      let url = url_builders::pull_requests(&repo);
      let details_body = json::encode(&details);
      details_body
        .map_err(|err| RequestErr::new(ErrorKind::Other, err.description().to_owned()))
        .and_then(|details| {
          self.post(url, Some(details))
            .map_err(|_| RequestErr::new(ErrorKind::Other, "Request failed".to_owned()))
            .and_then(|res| deserialize(res).map_err(|err| RequestErr::new(ErrorKind::Other, err.description().to_owned())))
        })
    }

    fn create_from_issue(self, repo: Repository, details: CreatePullRequestFromIssue) -> Result<PullRequest, RequestErr> {
      let url = url_builders::pull_requests(&repo);
      let details_body = json::encode(&details);
      details_body
        .map_err(|err| RequestErr::new(ErrorKind::Other, err.description().to_owned()))
        .and_then(|details| {
          self.post(url, Some(details))
            .map_err(|_| RequestErr::new(ErrorKind::Other, "Request failed".to_owned()))
            .and_then(|res| deserialize(res).map_err(|err| RequestErr::new(ErrorKind::Other, err.description().to_owned())))
        })
    }

    fn update_pull_request(self, pull_request: PullRequestReference, update: PullRequestUpdate) -> Result<PullRequest, RequestErr> {
      let url = url_builders::pull_request_at(&pull_request.repo, &pull_request.pull_request_id);
      let update_body = json::encode(&update);
      update_body
        .map_err(|err| RequestErr::new(ErrorKind::Other, err.description().to_owned()))
        .and_then(|update| {
          self.patch(url, Some(update))
            .map_err(|_| RequestErr::new(ErrorKind::Other, "Request failed".to_owned()))
            .and_then(|res| deserialize(res).map_err(|err| RequestErr::new(ErrorKind::Other, err.description().to_owned())))
        })
    }

    fn list_commits(self, pull_request: PullRequestReference) -> Result<Vec<Commit>, RequestErr> {
      let url = url_builders::pull_request_commits(&pull_request.repo, &pull_request.pull_request_id);
      self.get(url, None)
        .map_err(|_| RequestErr::new(ErrorKind::Other, "Request failed".to_owned()))
        .and_then(|res| deserialize(res).map_err(|err| RequestErr::new(ErrorKind::Other, err.description().to_owned())))
    }

    fn list_files(self, pull_request: PullRequestReference) -> Result<Vec<PullRequestFile>, RequestErr> {
      let url = url_builders::pull_request_files(&pull_request.repo, &pull_request.pull_request_id);
      self.get(url, None)
        .map_err(|_| RequestErr::new(ErrorKind::Other, "Request failed".to_owned()))
        .and_then(|res| deserialize(res).map_err(|err| RequestErr::new(ErrorKind::Other, err.description().to_owned())))
    }

    fn get_merged(self, pull_request: PullRequestReference) -> Result<MergedStatus, RequestErr> {
      // TODO:
      Err(RequestErr::new(ErrorKind::Other, "not implemented".to_owned()))
    }

    fn merge(self, pull_request: PullRequestReference, merge_request: Option<MergeRequest>) -> Result<MergedResult, RequestErr> {
      // TODO:
      Err(RequestErr::new(ErrorKind::Other, "not implemented".to_owned()))
    }
  }

  #[cfg(test)]
  mod tests {
    use super::PullRequester;
    use types::Repository;
    use github_client::GithubClient;
    use hyper::header::Authorization;
    use std::env;

    //#[test]
    fn list_works() {
      let token = env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap();
      let client = GithubClient::new(Some(Authorization("token ".to_owned() + &token)));
      let pull_requests = client.list(Repository { owner: "acmcarther".to_owned(), repo_name: "rust-roguelike".to_owned() }, None);
      println!("{:?}", pull_requests)
    }

    //#[test]
    fn get_pr_works() {
      let token = env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap();
      let client = GithubClient::new(Some(Authorization("token ".to_owned() + &token)));
      let pull_requests = client.get_pr(Repository { owner: "acmcarther".to_owned(), repo_name: "rust-roguelike".to_owned() }, 1);
      println!("{:?}", pull_requests)
    }
  }
}
