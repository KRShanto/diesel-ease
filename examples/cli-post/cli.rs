use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[clap(version, author, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommand {
    #[clap(about = "Create a new post")]
    Create {
        #[clap(short, long, help = "Title of the post")]
        title: String,
        #[clap(short, long, help = "Body of the post")]
        body: String,
    },

    #[clap(about = "List/Get all posts (multiple)")]
    List,

    #[clap(about = "Get a post by id (single)")]
    Get {
        #[clap(short, long, help = "Id of the post")]
        id: i32,
    },

    #[clap(about = "Get posts by published status (multiple)")]
    GetByPublished {
        #[clap(short, long, help = "Published status of the post")]
        published: bool,
    },

    #[clap(about = "Get posts by title (multiple)")]
    GetByTitle {
        #[clap(short, long, help = "Title of the post")]
        title: String,
    },

    #[clap(about = "Update a post ")]
    Update {
        #[clap(short, long, help = "Get post by Id")]
        id: i32,

        #[clap(short, long, help = "New Title of the post")]
        title: Option<String>,

        #[clap(short, long, help = "New Body of the post")]
        body: Option<String>,

        #[clap(short, long, help = "New Published status of the post")]
        published: Option<bool>,
    },

    #[clap(about = "Delete a post")]
    Delete {
        #[clap(short, long, help = "Id of the post")]
        id: i32,
    },

    #[clap(about = "Delete all posts")]
    DeleteAll,

    #[clap(about = "Delete by published status")]
    DeleteByPublished {
        #[clap(short, long, help = "Published status of posts")]
        published: bool,
    },

    #[clap(about = "Delete by title ")]
    DeleteByTitle {
        #[clap(short, long, help = "Title of the post")]
        title: String,
    },
}
