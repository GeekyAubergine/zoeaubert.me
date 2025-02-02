use chrono::{DateTime, Utc};
use inquire::{DateSelect, Select, Text};

use crate::domain::models::slug::Slug;
use crate::domain::models::tag::Tag;
use crate::domain::state::State;

use crate::infrastructure::utils::date::parse_date;
use crate::prelude::*;

const CONTENT_TYPE_BLOG_POST: &str = "Blog Post";
const CONTENT_TYPE_MICRO: &str = "Micro";

const CONTENT_TYPES: [&str; 2] = [CONTENT_TYPE_BLOG_POST, CONTENT_TYPE_MICRO];

fn get_date() -> Result<DateTime<Utc>> {
    let date = DateSelect::new("Date").prompt()?;

    let time = Text::new("Time").prompt()?;

    let date_as_string = format!("{}T{}:00", date, time);

    parse_date(&date_as_string)
}

fn get_tags() -> Result<Vec<Tag>> {
    let tags = Text::new("Tags").prompt()?;

    Ok(tags.split(',').map(|t| Tag::from_string(t)).collect())
}

fn get_text(prompt: &str) -> Result<String> {
    let text = Text::new(prompt).prompt()?;

    Ok(text)
}

async fn create_blog_post(state: &impl State) -> Result<()> {
    let date = get_date()?;
    let title = get_text("Title")?;
    let slug = get_text("Slug")?;
    let description = get_text("Description")?;
    let tags = get_tags()?;
    Ok(())
}

pub async fn create_content(state: &impl State) -> Result<()> {
    println!("--------------------------------");

    let content_type_option = Select::new(
        "What type of content would you like to create?",
        CONTENT_TYPES.to_vec(),
    )
    .prompt()?;

    match content_type_option {
        CONTENT_TYPE_BLOG_POST => create_blog_post(state).await,
        CONTENT_TYPE_MICRO => {
            println!("Creating a new micro post");
            Ok(())
        }
        _ => {
            println!("Invalid content type");
            Ok(())
        }
    }
}
