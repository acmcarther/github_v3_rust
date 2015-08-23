pub use self::github_client::GithubClient;

mod github_client {
  use hyper::Client;
  use hyper::header::{Authorization, Connection, Scheme, UserAgent};
  use hyper::method::Method;
  use hyper::client::response::Response;
  use hyper::error::Error;

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
    pub fn get(self, url: Url) -> Result<Response, Error> {
      let token = self.token.clone();
      let request = self.client.get(&url)
        .header(UserAgent("CatalystBot".to_owned()))
        .header(Connection::close());

      match token {
        Some(authorization) => request.header(authorization).send(),
        None => request.send()
      }
    }

    pub fn post(self, url: Url, body: Body) -> Result<Response, Error> {
      let token = self.token.clone();
      let request = self.client.post(&url)
        .header(UserAgent("CatalystBot".to_owned()))
        .header(Connection::close())
        .body(&body);

      match token {
        Some(authorization) => request.header(authorization).send(),
        None => request.send()
      }
    }

    pub fn put(self, url: Url, body: Body) -> Result<Response, Error> {
      let token = self.token.clone();
      let request = self.client.put(&url)
        .header(UserAgent("CatalystBot".to_owned()))
        .header(Connection::close())
        .body(&body);

      match token {
        Some(authorization) => request.header(authorization).send(),
        None => request.send()
      }
    }

    pub fn patch(self, url: Url, body: Body) -> Result<Response, Error> {
      let token = self.token.clone();
      let request = self.client.request(Method::Patch, &url)
        .header(UserAgent("CatalystBot".to_owned()))
        .header(Connection::close())
        .body(&body);

      match token {
        Some(authorization) => request.header(authorization).send(),
        None => request.send()
      }
    }
  }
}
