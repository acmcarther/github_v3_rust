pub mod types;

mod pull_requests {
  use hyper::header::Scheme;

  use github_client::GithubClient;

  use std::any::Any;
  use std::io::Read;

  use rustc_serialize::{
    json
  };

  use types::{
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
    GitTm,
    SortDirection,
    Repository,
  };

  use pull_requests::types::{
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

  use repos::types::{
    Repo,
    RepoPermissions
  };

  use commits::types::Commit;

  use users::types::User;

  pub trait PullRequester {
    fn list(self, repo: Repository, query: Option<PullRequestQuery>) -> Result<Vec<PullRequest>, RequestErr>;
    fn get_pr(self, repo: Repository, pr_id: PullRequestId) -> Result<PullRequest, RequestErr>;
    fn create_raw(self, repo: Repository, title: PullRequestTitle, head: HeadQuery, base: BranchName, message: Message) -> Result<PullRequest, RequestErr>;
    fn create_from_issue(self, repo: Repository, head: HeadQuery, base: BranchName, issue: IssueId) -> Result<PullRequest, RequestErr>;
    fn update_pull_request(self, pull_request: PullRequestReference, update: PullRequestUpdate) -> Result<UpdatedPullRequest, RequestErr>;
    fn list_commits(self, pull_request: PullRequestReference) -> Result<Vec<Commit>, RequestErr>;
    fn list_files(self, pull_request: PullRequestReference) -> Result<Vec<PullRequestFile>, RequestErr>;
    fn get_merged(self, pull_request: PullRequestReference) -> Result<MergedStatus, RequestErr>;
    fn merge(self, pull_request: PullRequestReference, merge_request: Option<MergeRequest>) -> Result<MergedResult, RequestErr>;
  }

  impl<S: Scheme + Any> PullRequester for GithubClient<S> where S::Err: 'static {
    fn list(self, repo: Repository, query: Option<PullRequestQuery>) -> Result<Vec<PullRequest>, RequestErr> {
      let url = "https://api.github.com/repos/".to_owned() + &repo.owner + "/" + &repo.repo_name + "/pulls";
      self.get(url)
        .map(|mut response| {
          let mut buf = String::new();
          let _ = response.read_to_string(&mut buf);
          println!("recv: {}", buf);
          json::decode(&buf).unwrap()
        })
        .map_err(|_| "Request failed".to_owned())
    }

    fn get_pr(self, repo: Repository, pr_id: PullRequestId) -> Result<PullRequest, RequestErr> {
      let pr_string = pr_id.to_string();
      let url = "https://api.github.com/repos/".to_owned() + &repo.owner + "/" + &repo.repo_name + "/pulls/" + &pr_string;
      self.get(url)
        .map(|mut response| {
          let mut buf = String::new();
          let _ = response.read_to_string(&mut buf);
          println!("recv: {}", buf);
          json::decode(&buf).unwrap()
        })
        .map_err(|_| "Request failed".to_owned())
    }

    fn create_raw(self, repo: Repository, title: PullRequestTitle, head: HeadQuery, base: BranchName, message: Message) -> Result<PullRequest, RequestErr> {
      Err("fuck".to_owned())
    }

    fn create_from_issue(self, repo: Repository, head: HeadQuery, base: BranchName, issue: IssueId) -> Result<PullRequest, RequestErr> {
      Err("fuck".to_owned())
    }

    fn update_pull_request(self, pull_request: PullRequestReference, update: PullRequestUpdate) -> Result<UpdatedPullRequest, RequestErr> {
      Err("fuck".to_owned())
    }

    fn list_commits(self, pull_request: PullRequestReference) -> Result<Vec<Commit>, RequestErr> {
      Err("fuck".to_owned())
    }

    fn list_files(self, pull_request: PullRequestReference) -> Result<Vec<PullRequestFile>, RequestErr> {
      Err("fuck".to_owned())
    }

    fn get_merged(self, pull_request: PullRequestReference) -> Result<MergedStatus, RequestErr> {
      Err("fuck".to_owned())
    }

    fn merge(self, pull_request: PullRequestReference, merge_request: Option<MergeRequest>) -> Result<MergedResult, RequestErr> {
      Err("fuck".to_owned())
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
