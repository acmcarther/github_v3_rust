pub use self::types::{
  Repo,
  RepoPermissions,
};

mod types {
  use types::{
    RepoName,
    BranchName,
    Url,
    GitTm,
  };

  use users::types::User;

  #[derive(RustcDecodable, Debug)]
  pub struct RepoPermissions {
    admin: bool,
    push: bool,
    pull: bool
  }

  #[derive(RustcDecodable, Debug)]
  pub struct Repo {
    id: u32,
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
    permissions: Option<RepoPermissions>
  }

}
