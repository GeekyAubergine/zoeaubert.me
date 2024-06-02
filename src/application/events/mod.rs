#[derive(Debug)]
pub enum Event {
    ServerBooted,

    GamesRepoLoadedFromArchive,
    GamesRepoUpdated,
    GamesRepoArchived,

    LegoRepoLoadedFromArchive,
    LegoRepoUpdated,
    LegoRepoArchived,

    StatusLolRepoLoadedFromArchive,
    StatusLolRepoUpdated,
    StatusLolRepoArchived,

    AboutRepoUpdated,

    FaqRepoUpdated,

    SillyNamesRepoUpdated,
    
    BlogPostsRepoUpdated,

}

impl Event {
    pub fn name(&self) -> &'static str {
        match self {
            Event::ServerBooted => "server.booted",

            Event::GamesRepoLoadedFromArchive => "games_repo.loaded_from_archive",
            Event::GamesRepoUpdated => "games_repo.updated",
            Event::GamesRepoArchived => "games_report.archived",

            Event::LegoRepoLoadedFromArchive => "lego_repo.loaded_from_archive",
            Event::LegoRepoUpdated => "lego_repo.updated",
            Event::LegoRepoArchived => "lego_repo.archived",

            Event::StatusLolRepoLoadedFromArchive => "status_lol_repo.loaded_from_archive",
            Event::StatusLolRepoUpdated => "status_lol_repo.updated",
            Event::StatusLolRepoArchived => "status_lol_repo.archived",

            Event::AboutRepoUpdated => "about_repo.updated",

            Event::FaqRepoUpdated => "faq_repo.updated",

            Event::SillyNamesRepoUpdated => "silly_names_repo.updated",

            Event::BlogPostsRepoUpdated => "blog_posts_repo.updated",

        }
    }
}
