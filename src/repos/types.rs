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
    id: u32, // TODO: This id
    owner: User,
    name: RepoName,
    full_name: RepoName,
    description: String,
    private: bool,
    fork: bool,
    url: Url,
    html_url: Url,
    archive_url: Url,
    assignees_url: Url,
    blobs_url: Url,
    branches_url: Url,
    clone_url: Url,
    collaborators_url: Url,
    comments_url: Url,
    commits_url: Url,
    compare_url: Url,
    contents_url: Url,
    contributors_url: Url,
    downloads_url: Url,
    events_url: Url,
    forks_url: Url,
    git_commits_url: Url,
    git_refs_url: Url,
    git_tags_url: Url,
    git_url: Url,
    hooks_url: Url,
    issue_comment_url: Url,
    issue_events_url: Url,
    issues_url: Url,
    keys_url: Url,
    labels_url: Url,
    languages_url: Url,
    merges_url: Url,
    milestones_url: Url,
    mirror_url: Option<Url>,
    notifications_url: Url,
    pulls_url: Url,
    releases_url: Url,
    ssh_url: Url,
    stargazers_url: Url,
    statuses_url: Url,
    subscribers_url: Url,
    subscription_url: Url,
    svn_url: Url,
    tags_url: Url,
    teams_url: Url,
    trees_url: Url,
    homepage: Option<Url>,
    language: Option<String>,
    forks_count: u32,
    stargazers_count: u32,
    watchers_count: u32,
    size: u32,
    default_branch: BranchName,
    open_issues_count: u32,
    has_issues: bool,
    has_wiki: bool,
    has_pages: bool,
    has_downloads: bool,
    pushed_at: GitTm,
    created_at: GitTm,
    updated_at: GitTm,
    permissions: Option<RepoPermissions>,
    subscribers_count: Option<u32>,
    organization: Option<Organization>,
    parent: Option<Box<Repo>>,
    source: Option<Box<Repo>>,
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
