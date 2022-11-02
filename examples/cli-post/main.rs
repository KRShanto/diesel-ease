// TODO: handle errors instead of panics (avoid .unwrap())

#[macro_use]
extern crate diesel;

use clap::Parser;
use cli::*;
use colored::*;
use diesel::prelude::*;
use models::*;

mod cli;
mod models;
mod schema;

fn main() {
    let cli = Cli::parse();
    let mut connection = establish_connection();

    match cli.subcommand {
        SubCommand::Create { title, body } => {
            Post::insert(
                &mut connection,
                NewPost {
                    title: &title,
                    body: &body,
                    published: false,
                },
            )
            .unwrap();

            println!("{}", "Post created successfully".blue().bold());
        }
        SubCommand::List => {
            let posts = Post::get_all(&mut connection).unwrap();
            println!("{:#?}", posts);
        }
        SubCommand::Get { id } => {
            let post = Post::get_by_id(&mut connection, &id).unwrap();
            println!("{:#?}", post);
        }
        SubCommand::GetByPublished { published } => {
            let posts = Post::get_by_published(&mut connection, &published).unwrap();
            println!("{:#?}", posts);
        }
        SubCommand::GetByTitle { title } => {
            let posts = Post::get_by_title(&mut connection, &title).unwrap();
            println!("{:#?}", posts);
        }
        SubCommand::Update {
            id,
            title,
            body,
            published,
        } => {
            if let Some(title) = title {
                Post::update_titles_by_id(&mut connection, &id, &title).unwrap();

                println!("{}", "Title updated successfully".blue().bold());
            }
            if let Some(body) = body {
                Post::update_bodys_by_id(&mut connection, &id, &body).unwrap();

                println!("{}", "Body updated successfully".blue().bold());
            }
            if let Some(published) = published {
                Post::update_publisheds_by_id(&mut connection, &id, &published).unwrap();

                println!("{}", "Published updated successfully".blue().bold());
            }
        }
        SubCommand::Delete { id } => {
            Post::delete_by_id(&mut connection, &id).unwrap();

            println!("{}", "Post deleted successfully".blue().bold());
        }
        SubCommand::DeleteAll => {
            Post::delete_all(&mut connection).unwrap();

            println!("{}", "All posts deleted successfully".blue().bold());
        }
        SubCommand::DeleteByPublished { published } => {
            Post::delete_by_published(&mut connection, &published).unwrap();

            println!("{}", "Posts deleted successfully".blue().bold());
        }
        SubCommand::DeleteByTitle { title } => {
            Post::delete_by_title(&mut connection, &title).unwrap();

            println!("{}", "Posts deleted successfully".blue().bold());
        }
    }
}

fn establish_connection() -> PgConnection {
    // read from env
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).unwrap()
}
