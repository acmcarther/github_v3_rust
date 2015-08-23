pub use self::types::{
  User
};

mod types {
  use types::{
    UserName,
    Url,
  };

  #[derive(RustcDecodable, Debug)]
  pub struct User {
    login: UserName,
    id: u32,
    avatar_url: Url,
    gravatar_id: String,  // TODO: What is this
    html_url: Url,
    followers_url: Url,
    following_url: Url,
    gists_url: Url,
    starred_url: Url,
    subscriptions_url: Url,
    organizations_url: Url,
    repos_url: Url,
    events_url: Url,
    received_events_url: Url,
    //type: String   // TODO: Custom decode for this reserved word
    site_admin: bool
  }

}
