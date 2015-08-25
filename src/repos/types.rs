pub use self::types::{
  Repo,
  RepoPermissions,
  RepoQuery,
  RepoVisibility,
  RepoAffiliations,
  RepoSortables,
  CreateRepository,
  EditRepository,
  ContributorsQuery,
  LanguagePile,
  Team,
  Tag,
  Branch,
  FullBranch,
  DeletedStatus,
  Organization,
  PublicReposQuery
};

mod types {
  use std::collections::HashMap;

  use rustc_serialize::{
    Decodable,
    Decoder,
    Encodable,
    Encoder,
  };

  use types::{
    RepoName,
    BranchName,
    Url,
    GitTm,
    SortDirection,
    Message,
    UserName,
  };

  use users::types::User;
  use commits::types::{
    CommitTreeNode,
    GithubCommit
  };

  #[derive(RustcDecodable, Debug)]
  pub struct RepoPermissions {
    admin: bool,
    push: bool,
    pull: bool
  }

  #[derive(RustcDecodable, Debug)]
  pub struct Repo {
    pub id: u32, // TODO: This id
    pub owner: User,
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
  pub struct Organization {
    login: UserName,
    id: u32,
    avatar_url: Url,
    gravatar_id: String,
    url: Url,
    html_url: Url,
    followers_url: Url,
    following_url: Url,
    gists_url: Url,
    subscriptions_url: Url,
    organizations_url: Url,
    repos_url: Url,
    events_url: Url,
    received_events_url: Url,
    // type: String   TODO: Custom decode for this key
    site_admin: bool
  }

  #[derive(RustcEncodable, Debug)]
  pub struct RepoQuery {
    visibility: Option<RepoVisibility>,
    affliation: Option<RepoAffiliations>,
    sort: Option<RepoSortables>,
    direction: Option<SortDirection>
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
    name: RepoName,
    description: Option<Message>,
    homepage: Option<Url>,
    private: Option<bool>,
    has_issues: Option<bool>,
    has_wiki: Option<bool>,
    has_downloads: Option<bool>,
    team_id: Option<u32>,
    auto_init: Option<bool>,
    gitignore_template: Option<String>,
    license_template: Option<String>
  }

  #[derive(RustcEncodable, Debug)]
  pub struct EditRepository {
    name: RepoName,
    description: Option<Message>,
    homepage: Option<Url>,
    private: Option<bool>,
    has_issues: Option<bool>,
    has_wiki: Option<bool>,
    has_downloads: Option<bool>,
    default_branch: Option<BranchName>
  }

  #[derive(RustcEncodable, Debug)]
  pub struct ContributorsQuery {
    anon: bool
  }

  #[derive(RustcDecodable, Debug)]
  pub struct LanguagePile(HashMap<String, u32>); // TODO: Types

  // TODO: Types
  #[derive(RustcDecodable, Debug)]
  pub struct Team {
    id: u32,
    url: Url,
    name: String,
    slug: String,
    description: String,
    privacy: String,
    permission: String,
    members_url: Url,
    repositories_url: Url,
  }

  // TODO: Types
  #[derive(RustcDecodable, Debug)]
  pub struct Tag {
    name: String,
    commit: CommitTreeNode,
    zipball_url: Url,
    tarball_url: Url
  }

  #[derive(RustcDecodable, Debug)]
  pub struct Branch {
    name: BranchName,
    commit: CommitTreeNode,
  }

  #[derive(RustcDecodable, Debug)]
  pub struct FullBranch {
    name: BranchName,
    commit: GithubCommit
    // TODO: _links
  }

  #[allow(dead_code)]
  pub enum DeletedStatus {
    Deleted,
    NotDeleted
  }

  #[derive(RustcEncodable, Debug)]
  pub struct PublicReposQuery {
    since: String
  }
}
