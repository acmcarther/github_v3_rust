pub use self::github_client::{
  GithubClient,
  SimpleClient
};

pub mod github_client {
  use hyper::Client;
  use hyper::header::{Accept, Authorization, Connection, qitem, Scheme, UserAgent};
  use hyper::mime::{Mime, TopLevel, SubLevel};
  use hyper::method::Method;
  use hyper::client::response::Response;
  use hyper::error::Error as HyperError;
  use hyper::client::{RequestBuilder, IntoUrl};
  use rustc_serialize::json::{DecoderError, EncoderError};
  use rustc_serialize::{
    json,
    Encodable,
    Decodable
  };

  use types::{GitErr, Url, Body};
  use std::io::Read;
  use std::any::Any;

  pub struct GithubClient<S: Scheme + Any> where S::Err: 'static {
    client: Client,
    token: Option<Authorization<S>>
  }

  fn net_err_to_git_err(err: HyperError) -> GitErr {
    GitErr::NetworkErr(err.to_string())
  }

  fn decode_err_to_git_err(err: DecoderError) -> GitErr {
    GitErr::DecodeErr(err.to_string())
  }

  fn encode_err_to_git_err(err: EncoderError) -> GitErr {
    GitErr::EncodeErr(err.to_string())
  }

  fn deserialize<S: Decodable>(response: Response) -> Result<S, GitErr> {
    let mut response = response;
    let mut buf = String::new();
    let _ = response.read_to_string(&mut buf);
    println!("recv: {}", buf);
    json::decode(&buf).map_err(decode_err_to_git_err)
  }

  // TODO: Remove
  pub trait SimpleClient {
    fn request_without_payload<D: Decodable>(&self, method: Method, url: Url) -> Result<D, GitErr>;
    fn request_with_payload<D: Decodable, E: Encodable>(&self, method: Method, url: Url, body: E) -> Result<D, GitErr>;
  }

  impl<S:Scheme + Any> GithubClient<S> where S::Err: 'static {
    pub fn new(token: Option<Authorization<S>>) -> GithubClient<S> {
      GithubClient { client: Client::new(), token: token }
    }

    fn request(&self, method: Method, url: Url, body: Option<Body>) -> Result<Response, GitErr> {
      let initial_request = self.client.request(method, &url);
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

  impl<S:Scheme + Any> SimpleClient for GithubClient<S> where S::Err: 'static {
    fn request_without_payload<D: Decodable>(&self, method: Method, url: Url) -> Result<D, GitErr> {
      self
        .request(method, url, None)
        .and_then(deserialize)
    }

    fn request_with_payload<D: Decodable, E: Encodable>(&self, method: Method, url: Url, body: E) -> Result<D, GitErr> {
      let encoded_body = json::encode(&body);
      encoded_body
        .map_err(encode_err_to_git_err)
        .and_then(|query| self.request(method, url, Some(query)))
        .and_then(deserialize)
    }

  }
}
