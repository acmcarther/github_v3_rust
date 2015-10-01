use types::Url;

pub type UserName = String;

#[derive(RustcDecodable, Debug, Clone)]
pub struct GithubUser {
  pub login: UserName,
  pub id: u32,
  pub avatar_url: Url,
  pub gravatar_id: String,  // TODO: What is this
  pub html_url: Url,
  pub followers_url: Url,
  pub following_url: Url,
  pub gists_url: Url,
  pub starred_url: Url,
  pub subscriptions_url: Url,
  pub organizations_url: Url,
  pub repos_url: Url,
  pub events_url: Url,
  pub received_events_url: Url,
  //type: String   // TODO: Custom decode for this reserved word
  pub site_admin: bool
}

#[derive(RustcDecodable, Debug, Clone)]
pub struct GitUser {
  pub name: String,
  pub email: String,
  pub username: Option<UserName>
}
