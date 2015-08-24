pub mod types;
pub mod url_builders;

mod pull_requests {
  use hyper::header::Scheme;
  use hyper::client::response::Response;
  use hyper::error::Error as HyperError;
  use rustc_serialize::Decodable;
  use rustc_serialize::json::{DecoderError, EncoderError};

  use github_client::GithubClient;

  use std::any::Any;
  use std::io::Read;
  use std::io::ErrorKind;

  use rustc_serialize::{
    json
  };

  use types::{
    GitErr,
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
    MergedStatus,
    PullRequestComment,
    PullRequestCommentQuery,
    CommentId,
    CreateComment,
    ReplyComment,
    EditComment,
    DeleteCommentStatus,
  };

  use pull_requests::url_builders;

  use commits::types::GithubCommit;

  fn deserialize<S: Decodable>(response: Response) -> Result<S, DecoderError> {
    let mut response = response;
    let mut buf = String::new();
    let _ = response.read_to_string(&mut buf);
    println!("recv: {}", buf);
    json::decode(&buf)
  }

  fn decode_err_to_git_err(err: DecoderError) -> GitErr {
    GitErr::new(ErrorKind::Other, "Decode failed: ".to_owned() + &err.to_string())
  }

  fn encode_err_to_git_err(err: EncoderError) -> GitErr {
    GitErr::new(ErrorKind::Other, "Encode failed: ".to_owned() + &err.to_string())
  }

  fn net_err_to_git_err(err: HyperError) -> GitErr {
    GitErr::new(ErrorKind::Other, "Request failed: ".to_owned() + &err.to_string())
  }

  pub trait PullRequester {
    fn list(self, repo: Repository, query: Option<PullRequestQuery>) -> Result<Vec<PullRequest>, GitErr>;
    fn get_pr(self, repo: Repository, pr_id: PullRequestId) -> Result<PullRequest, GitErr>;
    fn create_raw(self, repo: Repository, details: CreatePullRequest) -> Result<PullRequest, GitErr>;
    fn create_from_issue(self, repo: Repository, details: CreatePullRequestFromIssue) -> Result<PullRequest, GitErr>;
    fn update_pull_request(self, pull_request: PullRequestReference, update: PullRequestUpdate) -> Result<PullRequest, GitErr>;
    fn list_commits(self, pull_request: PullRequestReference) -> Result<Vec<GithubCommit>, GitErr>;
    fn list_files(self, pull_request: PullRequestReference) -> Result<Vec<PullRequestFile>, GitErr>;
    #[allow(dead_code, unused_variables)]
    fn get_merged(self, pull_request: PullRequestReference) -> Result<MergedStatus, GitErr>;
    #[allow(dead_code, unused_variables)]
    fn merge(self, pull_request: PullRequestReference, merge_request: Option<MergeRequest>) -> Result<MergedResult, GitErr>;
    fn list_comments(self, pull_request: PullRequestReference) -> Result<Vec<PullRequestComment>, GitErr>;
    fn list_all_pull_request_comments(self, repo: Repository, query: PullRequestCommentQuery) -> Result<Vec<PullRequestComment>, GitErr>;
    fn get_single_comment(self, repo: Repository, comment_id: CommentId) -> Result<PullRequestComment, GitErr>;
    fn create_comment(self, pull_request: PullRequestReference, comment_details: CreateComment) -> Result<PullRequestComment, GitErr>;
    fn create_comment_reply(self, pull_request: PullRequestReference, comment_details: ReplyComment) -> Result<PullRequestComment, GitErr>;
    fn edit_comment(self, repo: Repository, comment_id: CommentId, body: EditComment) -> Result<PullRequestComment, GitErr>;
    #[allow(dead_code, unused_variables)]
    fn delete_comment(self, repo: Repository, comment_id: CommentId) -> Result<DeleteCommentStatus, GitErr>;
  }

  impl<S: Scheme + Any> PullRequester for GithubClient<S> where S::Err: 'static {
    fn list(self, repo: Repository, query: Option<PullRequestQuery>) -> Result<Vec<PullRequest>, GitErr> {
      let url = url_builders::pull_requests(&repo);
      let query_body = query.map(|query| json::encode(&query));
      match query_body {
        Some(query_res) => {
          query_res
            .map_err(encode_err_to_git_err)
            .and_then(|query| {
              self.get(url, Some(query))
                .map_err(net_err_to_git_err)
                .and_then(|res| deserialize(res).map_err(decode_err_to_git_err))
            })
        },
        None => {
          self.get(url, None)
            .map_err(net_err_to_git_err)
            .and_then(|res| deserialize(res).map_err(decode_err_to_git_err))
        }
      }
    }

    fn get_pr(self, repo: Repository, pr_id: PullRequestId) -> Result<PullRequest, GitErr> {
      let url = url_builders::pull_request_at(&repo, &pr_id);
      self.get(url, None)
        .map_err(net_err_to_git_err)
        .and_then(|res| deserialize(res).map_err(decode_err_to_git_err))
    }

    fn create_raw(self, repo: Repository, details: CreatePullRequest) -> Result<PullRequest, GitErr> {
      let url = url_builders::pull_requests(&repo);
      let details_body = json::encode(&details);
      details_body
        .map_err(encode_err_to_git_err)
        .and_then(|details| {
          self.post(url, Some(details))
            .map_err(net_err_to_git_err)
            .and_then(|res| deserialize(res).map_err(decode_err_to_git_err))
        })
    }

    fn create_from_issue(self, repo: Repository, details: CreatePullRequestFromIssue) -> Result<PullRequest, GitErr> {
      let url = url_builders::pull_requests(&repo);
      let details_body = json::encode(&details);
      details_body
        .map_err(encode_err_to_git_err)
        .and_then(|details| {
          self.post(url, Some(details))
            .map_err(net_err_to_git_err)
            .and_then(|res| deserialize(res).map_err(decode_err_to_git_err))
        })
    }

    fn update_pull_request(self, pull_request: PullRequestReference, update: PullRequestUpdate) -> Result<PullRequest, GitErr> {
      let url = url_builders::pull_request_at(&pull_request.repo, &pull_request.pull_request_id);
      let update_body = json::encode(&update);
      update_body
        .map_err(encode_err_to_git_err)
        .and_then(|update| {
          self.patch(url, Some(update))
            .map_err(net_err_to_git_err)
            .and_then(|res| deserialize(res).map_err(decode_err_to_git_err))
        })
    }

    fn list_commits(self, pull_request: PullRequestReference) -> Result<Vec<GithubCommit>, GitErr> {
      let url = url_builders::pull_request_commits(&pull_request.repo, &pull_request.pull_request_id);
      self.get(url, None)
        .map_err(net_err_to_git_err)
        .and_then(|res| deserialize(res).map_err(decode_err_to_git_err))
    }

    fn list_files(self, pull_request: PullRequestReference) -> Result<Vec<PullRequestFile>, GitErr> {
      let url = url_builders::pull_request_files(&pull_request.repo, &pull_request.pull_request_id);
      self.get(url, None)
        .map_err(net_err_to_git_err)
        .and_then(|res| deserialize(res).map_err(decode_err_to_git_err))
    }

    #[allow(dead_code, unused_variables)]
    fn get_merged(self, pull_request: PullRequestReference) -> Result<MergedStatus, GitErr> {
      // TODO:
      Err(GitErr::new(ErrorKind::Other, "not implemented".to_owned()))
    }

    #[allow(dead_code, unused_variables)]
    fn merge(self, pull_request: PullRequestReference, merge_request: Option<MergeRequest>) -> Result<MergedResult, GitErr> {
      // TODO:
      Err(GitErr::new(ErrorKind::Other, "not implemented".to_owned()))
    }

    fn list_comments(self, pull_request: PullRequestReference) -> Result<Vec<PullRequestComment>, GitErr> {
      let url = url_builders::pull_request_comments(&pull_request.repo, &pull_request.pull_request_id);
      self.get(url, None)
        .map_err(net_err_to_git_err)
        .and_then(|res| deserialize(res).map_err(decode_err_to_git_err))
    }

    fn list_all_pull_request_comments(self, repo: Repository, query: PullRequestCommentQuery) -> Result<Vec<PullRequestComment>, GitErr> {
      let url = url_builders::all_pull_request_comments(&repo);
      let query_body = json::encode(&query);
      query_body
        .map_err(encode_err_to_git_err)
        .and_then(|query| {
          self.get(url, Some(query))
            .map_err(net_err_to_git_err)
            .and_then(|res| deserialize(res).map_err(decode_err_to_git_err))
        })
    }

    fn get_single_comment(self, repo: Repository, comment_id: CommentId) -> Result<PullRequestComment, GitErr> {
      let url = url_builders::pull_request_comment_at(&repo, &comment_id);
      self.get(url, None)
        .map_err(net_err_to_git_err)
        .and_then(|res| deserialize(res).map_err(decode_err_to_git_err))
    }

    fn create_comment(self, pull_request: PullRequestReference, comment_details: CreateComment) -> Result<PullRequestComment, GitErr> {
      let url = url_builders::pull_request_comments(&pull_request.repo, &pull_request.pull_request_id);
      let comment_detail = json::encode(&comment_details);
      comment_detail
        .map_err(encode_err_to_git_err)
        .and_then(|comment_detail| {
          self.post(url, Some(comment_detail))
            .map_err(net_err_to_git_err)
            .and_then(|res| deserialize(res).map_err(decode_err_to_git_err))
        })
    }

    fn create_comment_reply(self, pull_request: PullRequestReference, comment_details: ReplyComment) -> Result<PullRequestComment, GitErr> {
      let url = url_builders::pull_request_comments(&pull_request.repo, &pull_request.pull_request_id);
      let comment_detail = json::encode(&comment_details);
      comment_detail
        .map_err(encode_err_to_git_err)
        .and_then(|body| {
          self.post(url, Some(body))
            .map_err(net_err_to_git_err)
            .and_then(|res| deserialize(res).map_err(decode_err_to_git_err))
        })
    }

    fn edit_comment(self, repo: Repository, comment_id: CommentId, body: EditComment) -> Result<PullRequestComment, GitErr> {
      let url = url_builders::pull_request_comment_at(&repo, &comment_id);
      let body = json::encode(&body);
      body
        .map_err(encode_err_to_git_err)
        .and_then(|body| {
          self.patch(url, Some(body))
            .map_err(net_err_to_git_err)
            .and_then(|res| deserialize(res).map_err(decode_err_to_git_err))
        })
    }

    #[allow(dead_code, unused_variables)]
    fn delete_comment(self, repo: Repository, comment_id: CommentId) -> Result<DeleteCommentStatus, GitErr> {
      // TODO:
      Err(GitErr::new(ErrorKind::Other, "not implemented".to_owned()))
    }
  }

  #[cfg(test)]
  #[allow(dead_code)]
  mod tests {
    use super::PullRequester;
    use types::{SortDirection, Repository};
    use github_client::GithubClient;
    use hyper::header::Authorization;
    use std::env;
    use pull_requests::types::{
      CreatePullRequest,
      PullRequestState,
      PullRequestUpdate,
      PullRequestReference,
      PullRequestCommentQuery,
      PullRequestCommentSortable
    };

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

    //#[test]
    fn create_pr_works() {
      let token = env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap();
      let client = GithubClient::new(Some(Authorization("token ".to_owned() + &token)));
      let pull_requests = client.create_raw(Repository { owner: "pt-195".to_owned(), repo_name: "test".to_owned() }, CreatePullRequest {title: "Github api test".to_owned(), head: "t1".to_owned(), base: "master".to_owned(), body: Some("This is my test message".to_owned())});
      println!("{:?}", pull_requests)
    }

    //#[test]
    fn update_pr_works() {
      let token = env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap();
      let client = GithubClient::new(Some(Authorization("token ".to_owned() + &token)));
      let pull_requests = client.update_pull_request(PullRequestReference{ pull_request_id: 1, repo:Repository { owner: "pt-195".to_owned(), repo_name: "test".to_owned() }}, PullRequestUpdate {title: Some("test update title open".to_owned()), body: Some("Some new body open".to_owned()), state: Some(PullRequestState::Open)});
      println!("{:?}", pull_requests)
    }

    //#[test]
    fn list_commits_works() {
      let token = env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap();
      let client = GithubClient::new(Some(Authorization("token ".to_owned() + &token)));
      let pull_requests = client.list_commits(PullRequestReference{ pull_request_id: 1, repo:Repository { owner: "pt-195".to_owned(), repo_name: "test".to_owned() }});
      println!("{:?}", pull_requests)
    }

    //#[test]
    fn list_files_works() {
      let token = env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap();
      let client = GithubClient::new(Some(Authorization("token ".to_owned() + &token)));
      let pull_requests = client.list_files(PullRequestReference{ pull_request_id: 1, repo:Repository { owner: "pt-195".to_owned(), repo_name: "test".to_owned() }});
      println!("{:?}", pull_requests)
    }

    //#[test]
    fn list_comments_works() {
      let token = env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap();
      let client = GithubClient::new(Some(Authorization("token ".to_owned() + &token)));
      let pull_requests = client.list_comments(PullRequestReference{ pull_request_id: 1, repo:Repository { owner: "pt-195".to_owned(), repo_name: "test".to_owned() }});
      println!("{:?}", pull_requests)
    }

    //#[test]
    fn list_all_pull_request_comments_works() {
      let token = env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap();
      let client = GithubClient::new(Some(Authorization("token ".to_owned() + &token)));
      let pull_requests = client.list_all_pull_request_comments(Repository { owner: "pt-195".to_owned(), repo_name: "test".to_owned() }, PullRequestCommentQuery {sort: Some(PullRequestCommentSortable::Created), direction: Some(SortDirection::Descending), since: None});
      println!("{:?}", pull_requests)
    }

    //#[test]
    fn pull_request_comment_at_works() {
      let token = env::var("CATALYST_GITHUB_OAUTH_TOKEN").unwrap();
      let client = GithubClient::new(Some(Authorization("token ".to_owned() + &token)));
      let pull_requests = client.get_single_comment(Repository { owner: "pt-195".to_owned(), repo_name: "test".to_owned() }, 37711288);
      println!("{:?}", pull_requests)
    }
  }
}
