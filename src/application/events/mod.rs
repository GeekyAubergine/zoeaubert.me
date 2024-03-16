#[derive(Debug)]
pub enum Event {
    LoadedFromArchive,
    LegoRepoUpdated,
    GamesRepoUpdated,
    StatusLolRepoUpdated,
    AboutRepoUpdated,
    FaqRepoUpdated,
    SillyNamesRepoUpdated,
    BlogPostsRepoUpdated,

    GamesRepoArchived,
    LegoRepoArchived,
    StatusLolRepoArchived,
}

impl Event {
    pub fn name(&self) -> &'static str {
        match self {
            Event::LoadedFromArchive => "loaded_from_archive",
            Event::LegoRepoUpdated => "lego_repo.updated",
            Event::GamesRepoUpdated => "games_repo.updated",
            Event::StatusLolRepoUpdated => "status_lol_repo.updated",
            Event::AboutRepoUpdated => "about_repo.updated",
            Event::FaqRepoUpdated => "faq_repo.updated",
            Event::SillyNamesRepoUpdated => "silly_names_repo.updated",
            Event::BlogPostsRepoUpdated => "blog_posts_repo.updated",

            Event::GamesRepoArchived => "games_report.archived",
            Event::LegoRepoArchived => "lego_repo.archived",
            Event::StatusLolRepoArchived => "status_lol_repo.archived",
        }
    }

    pub fn loaded_from_archive() -> Self {
        Self::LoadedFromArchive
    }

    pub fn lego_repo_updated() -> Self {
        Self::LegoRepoUpdated
    }

    pub fn games_repo_updated() -> Self {
        Self::GamesRepoUpdated
    }

    pub fn status_lol_repo_updated() -> Self {
        Self::StatusLolRepoUpdated
    }

    pub fn about_repo_updated() -> Self {
        Self::AboutRepoUpdated
    }

    pub fn faq_repo_updated() -> Self {
        Self::FaqRepoUpdated
    }

    pub fn games_repo_archived() -> Self {
        Self::GamesRepoArchived
    }

    pub fn lego_repo_archived() -> Self {
        Self::LegoRepoArchived
    }

    pub fn status_lol_repo_archived() -> Self {
        Self::StatusLolRepoArchived
    }

    pub fn silly_names_repo_updated() -> Self {
        Self::SillyNamesRepoUpdated
    }

    pub fn blog_posts_repo_updated() -> Self {
        Self::BlogPostsRepoUpdated
    }
}
