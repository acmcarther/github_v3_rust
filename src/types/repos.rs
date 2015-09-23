use rustc_serialize::{
  Decodable,
  Decoder,
  Encodable,
  Encoder,
};

use types::{
  Url,
  BranchName,
  SortDirection,
  GitTm,
  Message,
  Organization,
};

use types::users::{
  GitUser,
  UserName,
  GithubUser
};

pub type RepoName = String;

#[derive(Debug)]
pub struct Repository {
  pub owner: UserName,
  pub repo_name: RepoName
}

#[derive(RustcDecodable, Debug)]
pub struct RepoPermissions {
  pub admin: bool,
  pub push: bool,
  pub pull: bool
}

#[derive(RustcDecodable, Debug)]
pub struct Repo {
  pub id: u32, // TODO: This id
  pub owner: GithubUser,
  pub name: RepoName,
  pub full_name: RepoName,
  pub description: String,
  pub private: bool,
  pub fork: bool,
  pub url: Url,
  pub html_url: Url,
  pub archive_url: Url,
  pub assignees_url: Url,
  pub blobs_url: Url,
  pub branches_url: Url,
  pub clone_url: Url,
  pub collaborators_url: Url,
  pub comments_url: Url,
  pub commits_url: Url,
  pub compare_url: Url,
  pub contents_url: Url,
  pub contributors_url: Url,
  pub downloads_url: Url,
  pub events_url: Url,
  pub forks_url: Url,
  pub git_commits_url: Url,
  pub git_refs_url: Url,
  pub git_tags_url: Url,
  pub git_url: Url,
  pub hooks_url: Url,
  pub issue_comment_url: Url,
  pub issue_events_url: Url,
  pub issues_url: Url,
  pub keys_url: Url,
  pub labels_url: Url,
  pub languages_url: Url,
  pub merges_url: Url,
  pub milestones_url: Url,
  pub mirror_url: Option<Url>,
  pub notifications_url: Url,
  pub pulls_url: Url,
  pub releases_url: Url,
  pub ssh_url: Url,
  pub stargazers_url: Url,
  pub statuses_url: Url,
  pub subscribers_url: Url,
  pub subscription_url: Url,
  pub svn_url: Url,
  pub tags_url: Url,
  pub teams_url: Url,
  pub trees_url: Url,
  pub homepage: Option<Url>,
  pub language: Option<String>,
  pub forks_count: u32,
  pub stargazers_count: u32,
  pub watchers_count: u32,
  pub size: u32,
  pub default_branch: BranchName,
  pub open_issues_count: u32,
  pub has_issues: bool,
  pub has_wiki: bool,
  pub has_pages: bool,
  pub has_downloads: bool,
  pub pushed_at: GitTm,
  pub created_at: GitTm,
  pub updated_at: GitTm,
  pub permissions: Option<RepoPermissions>,
  pub subscribers_count: Option<u32>,
  pub organization: Option<Organization>,
  pub parent: Option<Box<Repo>>,
  pub source: Option<Box<Repo>>,
}

#[derive(RustcDecodable, Debug)]
pub struct LegacyRepo {
  pub id: u32, // TODO: This id
  pub owner: GitUser,
  pub name: RepoName,
  pub full_name: RepoName,
  pub description: String,
  pub private: bool,
  pub fork: bool,
  pub url: Url,
  pub html_url: Url,
  pub archive_url: Url,
  pub assignees_url: Url,
  pub blobs_url: Url,
  pub branches_url: Url,
  pub clone_url: Url,
  pub collaborators_url: Url,
  pub comments_url: Url,
  pub commits_url: Url,
  pub compare_url: Url,
  pub contents_url: Url,
  pub contributors_url: Url,
  pub downloads_url: Url,
  pub events_url: Url,
  pub forks_url: Url,
  pub git_commits_url: Url,
  pub git_refs_url: Url,
  pub git_tags_url: Url,
  pub git_url: Url,
  pub hooks_url: Url,
  pub issue_comment_url: Url,
  pub issue_events_url: Url,
  pub issues_url: Url,
  pub keys_url: Url,
  pub labels_url: Url,
  pub languages_url: Url,
  pub merges_url: Url,
  pub milestones_url: Url,
  pub mirror_url: Option<Url>,
  pub notifications_url: Url,
  pub pulls_url: Url,
  pub releases_url: Url,
  pub ssh_url: Url,
  pub stargazers_url: Url,
  pub statuses_url: Url,
  pub subscribers_url: Url,
  pub subscription_url: Url,
  pub svn_url: Url,
  pub tags_url: Url,
  pub teams_url: Url,
  pub trees_url: Url,
  pub homepage: Option<Url>,
  pub language: Option<String>,
  pub forks_count: u32,
  pub stargazers_count: u32,
  pub watchers_count: u32,
  pub size: u32,
  pub default_branch: BranchName,
  pub open_issues_count: u32,
  pub has_issues: bool,
  pub has_wiki: bool,
  pub has_pages: bool,
  pub has_downloads: bool,
  pub pushed_at: u32,
  pub created_at: u32,
  pub updated_at: GitTm,
  pub permissions: Option<RepoPermissions>,
  pub subscribers_count: Option<u32>,
  pub organization: Option<Organization>,
  pub parent: Option<Box<Repo>>,
  pub source: Option<Box<Repo>>,
}

#[derive(RustcEncodable, Debug)]
pub struct RepoQuery {
  pub visibility: Option<RepoVisibility>,
  pub affliation: Option<RepoAffiliations>,
  pub sort: Option<RepoSortables>,
  pub direction: Option<SortDirection>
}

#[derive(Debug)]
pub enum RepoVisibility {
  Public,
  Private,
  All
}

custom_enum_decode_encode!(
  RepoVisibility [
    "public" <=> [RepoVisibility::Public],
    "private" <=> [RepoVisibility::Private],
    "all" <=> [RepoVisibility::All],
  ]
);

// TODO: Make this a proper product type
pub type RepoAffiliations = String;

#[derive(Debug)]
pub enum RepoSortables {
  Updated,
  Pushed,
  FullName,
}

custom_enum_decode_encode!(
  RepoSortables [
    "updated" <=> [RepoSortables::Updated],
    "pushed" <=> [RepoSortables::Pushed],
    "full_name" <=> [RepoSortables::FullName],
  ]
);

#[derive(RustcEncodable, Debug)]
pub struct CreateRepository {
  pub name: RepoName,
  pub description: Option<Message>,
  pub homepage: Option<Url>,
  pub private: Option<bool>,
  pub has_issues: Option<bool>,
  pub has_wiki: Option<bool>,
  pub has_downloads: Option<bool>,
  pub team_id: Option<u32>,
  pub auto_init: Option<bool>,
  pub gitignore_template: Option<String>,
  pub license_template: Option<String>
}

#[derive(RustcEncodable, Debug)]
pub struct EditRepository {
  pub name: RepoName,
  pub description: Option<Message>,
  pub homepage: Option<Url>,
  pub private: Option<bool>,
  pub has_issues: Option<bool>,
  pub has_wiki: Option<bool>,
  pub has_downloads: Option<bool>,
  pub default_branch: Option<BranchName>
}

#[derive(RustcEncodable, Debug)]
pub struct PublicReposQuery {
  pub since: String
}
