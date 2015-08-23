pub use self::github_client::GithubClient;

mod github_client {
  use hyper::Client;
  use hyper::header::{Authorization, Connection, Scheme, UserAgent};
  use hyper::method::Method;
  use hyper::client::response::Response;
  use hyper::error::Error;
  use hyper::client::{RequestBuilder, IntoUrl};

  use types::{Url, Body};
  use std::any::Any;

  pub struct GithubClient<S: Scheme + Any> where S::Err: 'static {
    client: Client,
    token: Option<Authorization<S>>
  }

  impl<S:Scheme + Any> GithubClient<S> where S::Err: 'static {
    pub fn new(token: Option<Authorization<S>>) -> GithubClient<S> {
      GithubClient { client: Client::new(), token: token }
    }

    fn buildCommonRequest<'a, U: IntoUrl>(&self, request: RequestBuilder<'a, U>) -> RequestBuilder<'a, U> {
      let common_request =
        request
          .header(UserAgent("CatalystBot".to_owned()))
          .header(Connection::close());

      let token = self.token.clone();
      match token {
        Some(authorization) => common_request.header(authorization),
        None => common_request
      }
    }

    pub fn get(self, url: Url) -> Result<Response, Error> {
      let initial_request = self.client.get(&url);
      self.buildCommonRequest(initial_request).send()
    }

    pub fn post(self, url: Url, body: Body) -> Result<Response, Error> {
      let initial_request = self.client.post(&url);
      self.buildCommonRequest(initial_request).body(&body).send()
    }

    pub fn put(self, url: Url, body: Body) -> Result<Response, Error> {
      let initial_request = self.client.put(&url);
      self.buildCommonRequest(initial_request).body(&body).send()
    }

    pub fn patch(self, url: Url, body: Body) -> Result<Response, Error> {
      let initial_request = self.client.request(Method::Patch, &url);
      self.buildCommonRequest(initial_request).body(&body).send()
    }
  }
}
