pub use self::github_client::GithubClient;

mod github_client {
  use hyper::Client;
  use hyper::header::{Accept, Authorization, Connection, qitem, Scheme, UserAgent};
  use hyper::mime::{Mime, TopLevel, SubLevel};
  use hyper::method::Method;
  use hyper::client::response::Response;
  use hyper::error::Error as HyperError;
  use hyper::client::{RequestBuilder, IntoUrl};

  use types::{GitErr, Url, Body};
  use std::io::ErrorKind;
  use std::any::Any;

  pub struct GithubClient<S: Scheme + Any> where S::Err: 'static {
    client: Client,
    token: Option<Authorization<S>>
  }

  fn net_err_to_git_err(err: HyperError) -> GitErr {
    GitErr::new(ErrorKind::Other, "Request failed: ".to_owned() + &err.to_string())
  }

  impl<S:Scheme + Any> GithubClient<S> where S::Err: 'static {
    pub fn new(token: Option<Authorization<S>>) -> GithubClient<S> {
      GithubClient { client: Client::new(), token: token }
    }

    pub fn get(self, url: Url, body: Option<Body>) -> Result<Response, GitErr> {
      let initial_request = self.client.get(&url);
      self.build_common_request(initial_request, body.unwrap_or("".to_owned()).as_ref())
    }

    pub fn post(self, url: Url, body: Option<Body>) -> Result<Response, GitErr> {
      let initial_request = self.client.post(&url);
      self.build_common_request(initial_request, body.unwrap_or("".to_owned()).as_ref())
    }

    pub fn put(self, url: Url, body: Option<Body>) -> Result<Response, GitErr> {
      let initial_request = self.client.put(&url);
      self.build_common_request(initial_request, body.unwrap_or("".to_owned()).as_ref())
    }

    pub fn patch(self, url: Url, body: Option<Body>) -> Result<Response, GitErr> {
      let initial_request = self.client.request(Method::Patch, &url);
      self.build_common_request(initial_request, body.unwrap_or("".to_owned()).as_ref())
    }

    fn build_common_request<'a, U: IntoUrl>(&self, request: RequestBuilder<'a, U>, body: &'a str) -> Result<Response, GitErr> {
      let common_request =
        request
          .header(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Ext("vnd.github.v3+json".to_owned()), vec![]))]))
          .header(UserAgent("CatalystBot".to_owned()))
          .header(Connection::close());

      let token = self.token.clone();
      let auth_request = match token {
        Some(authorization) => common_request.header(authorization),
        None => common_request
      };

      auth_request.body(body).send().map_err(net_err_to_git_err)
    }

  }
}
