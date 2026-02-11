#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigSteam {
    pub api_key: &'static str,
    pub user_id: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigBrickset {
    pub api_key: &'static str,
    pub username: &'static str,
    pub password: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigMastodon {
    pub account_id: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigBunnyCdn {
    pub url: &'static str,
    pub access_key: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigTMDB {
    pub key: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub steam: ConfigSteam,
    pub brickset: ConfigBrickset,
    pub mastodon: ConfigMastodon,
    pub bunny_cdn: ConfigBunnyCdn,
    pub tmdb: ConfigTMDB,

    pub cdn_url: &'static str,
}

#[cfg(feature = "env-variables")]
const fn load() -> Config {
    Config {
        steam: ConfigSteam {
            api_key: env!("STEAM_API_KEY"),
            user_id: env!("STEAM_ID"),
        },
        brickset: ConfigBrickset {
            api_key: env!("BRICKSET_API_KEY"),
            username: env!("BRICKSET_USERNAME"),
            password: env!("BRICKSET_PASSWORD"),
        },
        mastodon: ConfigMastodon {
            account_id: env!("MASTODON_ACCOUNT_ID"),
        },
        bunny_cdn: ConfigBunnyCdn {
            url: env!("BUNNY_CDN_URL"),
            access_key: env!("BUNNY_CDN_ACCESS_KEY"),
        },
        tmdb: ConfigTMDB {
            key: env!("TMDB_KEY"),
        },

        cdn_url: "https://cdn.geekyaubergine.com",
    }
}

#[cfg(not(feature = "env-variables"))]
const fn load() -> Config {
    use dotenvy_macro::dotenv;

    Config {
        steam: ConfigSteam {
            api_key: dotenv!("STEAM_API_KEY"),
            user_id: dotenv!("STEAM_ID"),
        },
        brickset: ConfigBrickset {
            api_key: dotenv!("BRICKSET_API_KEY"),
            username: dotenv!("BRICKSET_USERNAME"),
            password: dotenv!("BRICKSET_PASSWORD"),
        },
        mastodon: ConfigMastodon {
            account_id: dotenv!("MASTODON_ACCOUNT_ID"),
        },
        bunny_cdn: ConfigBunnyCdn {
            url: dotenv!("BUNNY_CDN_URL"),
            access_key: dotenv!("BUNNY_CDN_ACCESS_KEY"),
        },
        tmdb: ConfigTMDB {
            key: dotenv!("TMDB_KEY"),
        },

        cdn_url: "https://cdn.geekyaubergine.com",
    }
}

pub const CONFIG: Config = load();
