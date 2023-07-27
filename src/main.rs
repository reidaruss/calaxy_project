/*
    Created by Reid Russell
    Updated : 7/26/2023

    This is a simple REST API that allows users to create stories and comments on those stories.
*/

#[macro_use] extern crate rocket;

use core::result::Result;
use rocket_multipart_form_data::{MultipartFormDataOptions, MultipartFormData, MultipartFormDataField};
use rocket::State;
use serde::{Serialize, Deserialize};
use rocket::Data;
use rocket::http::ContentType;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use rocket::http::Status;
use std::process::Command;



#[derive(Serialize, Deserialize)]

struct Story {
    story_id: i32,
    username: String,
    story_content: String
}

#[derive(Serialize, Deserialize)]

struct User {
    user_id: i32,
    username: String,
    pass: String,
}

#[derive(Serialize, Deserialize)]

struct Comment {
    comment_id: i32,
    story_id: i32,
    username: String,
    comment_content: String
}

impl User {
    async fn insert_user_to_db(username: String,password:String, pool: &Pool<Postgres>) -> Result<(),sqlx::Error> {

        let _insert = sqlx::query_as!(
            User,
            "insert into users (username, pass) values ($1, $2)",
            username.clone(),
            password
        ).execute(pool)
        .await.expect("Unable to insert user");


        Ok(())
    }
    // async fn get_all_users( pool: &Pool<Postgres>) -> Result<Vec<User>,sqlx::Error> {
    async fn get_all_users( pool: &Pool<Postgres>) -> Vec<User> {
        let users: Vec<User> = sqlx::query_as!(
            User,
            r"select * from users"
        ).fetch_all(pool)
        .await.expect("Unable to get all users");



        users
    }

    async fn delete_user(username: String, pool: &Pool<Postgres>) -> Result<(),sqlx::Error> {

        // Remove all comments of the story before deleting it
        let _ = Comment::delete_comments_by_user(username.clone(), pool).await;
        let _ = Story::delete_stories_by_user(username.clone(), pool).await;

        let _insert = sqlx::query_as!(
            User,
            "DELETE FROM users WHERE username = $1",
            username
        ).execute(pool)
        .await.expect("Unable to delete comment");



        Ok(())
    }

}

impl Story {
    async fn insert_story_to_db(username: String, content:String, pool: &Pool<Postgres>) -> Result<(),sqlx::Error> {

        let _insert = sqlx::query_as!(
            Story,
            "insert into story (username, story_content) values ($1, $2)",
            username.clone(),
            content.clone()
        ).execute(pool)
        .await.expect("Unable to insert story");

        // Send message to external system upon story creation (requestbin)
        // curl -d '{ "username": {} }'   -H "Content-Type: application/json"   https://en9l94zz8cpcg.x.pipedream.net/

        Command::new("curl")
        .arg(format!("-d '{{ 'username': '{}' }}'", username))
        .arg("-H 'Content-Type: application/json' ")
        .arg("https://en9l94zz8cpcg.x.pipedream.net/")
        .spawn()
        .expect("curl failed");



        Ok(())
    }
    async fn get_stories_by_user(username: String, pool: &Pool<Postgres>) -> Vec<Story> {

        let stories: Vec<Story> = sqlx::query_as!(
            Story,
            "select * from story where username = $1",
            username.clone()
        ).fetch_all(pool)
        .await.expect("Unable to get stories by user");



        stories
    }

    async fn delete_story(story_id: i32, pool: &Pool<Postgres>) -> Result<(),sqlx::Error> {

        // Remove all comments of the story before deleting it
        let _ = Comment::delete_all_story_comments(story_id.clone(), pool).await;

        let _insert = sqlx::query_as!(
            Story,
            "DELETE FROM story WHERE story_id = $1",
            story_id
        ).execute(pool)
        .await.expect("Unable to delete comment");



        Ok(())
    }

    async fn delete_stories_by_user(username: String, pool: &Pool<Postgres>) -> Result<(),sqlx::Error> {

        // Delete comments from stories first
        let _ = sqlx::query_as!(Comment,
            "DELETE FROM comment
            WHERE story_id IN (
                SELECT story_id
                FROM story
                WHERE username = $1
            )",
            username.clone()

        ).execute(pool)
        .await.expect("Unable to delete stories by user");

        let _insert = sqlx::query_as!(
            Story,
            "DELETE FROM story WHERE username = $1",
            username
        ).execute(pool)
        .await.expect("Unable to delete stories by user");



        Ok(())
    }
}


impl Comment {
    async fn insert_comment_to_db(story_id:i32,username: String, content:String, pool: &Pool<Postgres>) -> Result<(),sqlx::Error> {

        let _insert = sqlx::query_as!(
            Comment,
            "insert into comment (story_id, username, comment_content) values ($1, $2, $3)",
            story_id,
            username.clone(),
            content
        ).execute(pool)
        .await.expect("Unable to insert story");



        Ok(())
    }
    async fn get_comments_by_story(story_id: i32, pool: &Pool<Postgres>) -> Vec<Comment> {

        let comments: Vec<Comment> = sqlx::query_as!(
            Comment,
            "select * from comment where story_id = $1",
            story_id
        ).fetch_all(pool)
        .await.expect("Unable to get stories by comment");



        comments
    }

    async fn get_comments_by_user(username: String, pool: &Pool<Postgres>) -> Vec<Comment> {

        let comments: Vec<Comment> = sqlx::query_as!(
            Comment,
            "select * from comment where username = $1",
            username
        ).fetch_all(pool)
        .await.expect("Unable to get stories by comment");



        comments
    }

    async fn delete_comments_by_user(username: String, pool: &Pool<Postgres>) -> Result<(),sqlx::Error> {

        let _insert = sqlx::query_as!(
            Comment,
            "DELETE FROM comment WHERE username = $1",
            username
        ).execute(pool)
        .await.expect("Unable to delete comments by user");



        Ok(())
    }

    async fn delete_comment(comment_id: i32, pool: &Pool<Postgres>) -> Result<(),sqlx::Error> {

        let _insert = sqlx::query_as!(
            Comment,
            "DELETE FROM comment WHERE comment_id = $1",
            comment_id
        ).execute(pool)
        .await.expect("Unable to delete comment");



        Ok(())
    }

    async fn delete_all_story_comments(story_id: i32, pool: &Pool<Postgres>) -> Result<(),sqlx::Error> {

        let _insert = sqlx::query_as!(
            Comment,
            "DELETE FROM comment WHERE story_id = $1",
            story_id
        ).execute(pool)
        .await.expect("Unable to delete comment");



        Ok(())
    }
}


#[get("/")]
fn index() -> &'static str {
    "This endpoint does nothing, try out the other endpoints."
}



// USERS

#[post("/",  data = "<data>")]
async fn create_user(pool: &State<Pool<Postgres>>,content_type: &ContentType , data: Data<'_>) -> Result<String, Status> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::text("username"),
        MultipartFormDataField::text("password")
        
        ]);
    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();
    let username = multipart_form_data.texts.remove("username").unwrap().remove(0).text.into();
    let password = multipart_form_data.texts.remove("password").unwrap().remove(0).text.into();

    

    let user = User::insert_user_to_db(username,password, &pool).await;

    match user {
        Ok(_user) => Ok("Success".to_string()),
        _ => Err(Status::NotFound)
    }
    



}


#[get("/")]
async fn get_all_users(pool: &State<Pool<Postgres>>) -> Result<String, Status> {


    // let user = User::insert_user_to_db(username,password, &pool).await;
    let res = User::get_all_users(&pool).await;

    let json_string = serde_json::to_string(&res).expect("Failed to serialize to JSON");



    match json_string{
        json_string => Ok(json_string),
        _ => Err(Status::NotFound)
    }
    
}


#[delete("/",  data = "<data>")]
async fn delete_user(pool: &State<Pool<Postgres>>,content_type: &ContentType , data: Data<'_>) -> Result<String, Status> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::text("username")
        
        ]);
    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();
    let username= multipart_form_data.texts.remove("username").unwrap().remove(0).text.into();

    // let user = User::insert_user_to_db(username,password, &pool).await;
    let res = User::delete_user(username,&pool).await;




    match res {
        Ok(_user) => Ok("Success".to_string()),
        _ => Err(Status::NotFound)
    }
    
}
// END USERS
// STORIES
#[get("/",  data = "<data>")]
async fn get_stories_by_user(pool: &State<Pool<Postgres>>,content_type: &ContentType , data: Data<'_>) -> Result<String, Status> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::text("username")
        
        ]);
    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();
    let username = multipart_form_data.texts.remove("username").unwrap().remove(0).text.into();

    // let user = User::insert_user_to_db(username,password, &pool).await;
    let res = Story::get_stories_by_user(username,&pool).await;

    let json_string = serde_json::to_string(&res).expect("Failed to serialize to JSON");



    match json_string{
        json_string => Ok(json_string),
        _ => Err(Status::NotFound)
    }
    
}


#[post("/",  data = "<data>")]
async fn create_story(pool: &State<Pool<Postgres>>,content_type: &ContentType , data: Data<'_>) -> Result<String, Status> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(
        vec![
            MultipartFormDataField::text("username"),
            MultipartFormDataField::text("content")
            ]
        );
    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();
    let uname = multipart_form_data.texts.remove("username").unwrap().remove(0).text.into();
    let content = multipart_form_data.texts.remove("content").unwrap().remove(0).text.into();

    // let s = Story {username: uname.unwrap().remove(0).text.into(), content: content.unwrap().remove(0).text.into()};


    let story = Story::insert_story_to_db(uname, content, &pool).await;

    match story {
        Ok(_user) => Ok("Success".to_string()),
        _ => Err(Status::NotFound)
    }
    
    
}

#[delete("/",  data = "<data>")]
async fn delete_story(pool: &State<Pool<Postgres>>,content_type: &ContentType , data: Data<'_>) -> Result<String, Status> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::text("story_id")
        
        ]);
    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();
    let story_id= multipart_form_data.texts.remove("story_id").unwrap().remove(0).text.parse::<i32>().unwrap();

    // let user = User::insert_user_to_db(username,password, &pool).await;
    let res = Story::delete_story(story_id,&pool).await;




    match res {
        Ok(_user) => Ok("Success".to_string()),
        _ => Err(Status::NotFound)
    }
    
}

#[delete("/",  data = "<data>")]
async fn delete_stories_by_user(pool: &State<Pool<Postgres>>,content_type: &ContentType , data: Data<'_>) -> Result<String, Status> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::text("username")
        
        ]);
    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();
    let username= multipart_form_data.texts.remove("username").unwrap().remove(0).text.into();

    // let user = User::insert_user_to_db(username,password, &pool).await;
    let res = Story::delete_stories_by_user(username,&pool).await;




    match res {
        Ok(_user) => Ok("Success".to_string()),
        _ => Err(Status::NotFound)
    }
    
}
// END STORIES

// COMMENTS

#[get("/",  data = "<data>")]
async fn get_comments_by_story(pool: &State<Pool<Postgres>>,content_type: &ContentType , data: Data<'_>) -> Result<String, Status> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::text("story_id")
        
        ]);
    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();
    let story_id= multipart_form_data.texts.remove("story_id").unwrap().remove(0).text.parse::<i32>().unwrap();

    // let user = User::insert_user_to_db(username,password, &pool).await;
    let res: Vec<Comment> = Comment::get_comments_by_story(story_id,&pool).await;

    let json_string = serde_json::to_string(&res).expect("Failed to serialize to JSON");



    match json_string{
        json_string => Ok(json_string),
        _ => Err(Status::NotFound)
    }
    
}

#[get("/",  data = "<data>")]
async fn get_comments_by_user(pool: &State<Pool<Postgres>>,content_type: &ContentType , data: Data<'_>) -> Result<String, Status> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::text("username")
        
        ]);
    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();
    let username= multipart_form_data.texts.remove("username").unwrap().remove(0).text.into();

    // let user = User::insert_user_to_db(username,password, &pool).await;
    let res: Vec<Comment> = Comment::get_comments_by_user(username,&pool).await;

    let json_string = serde_json::to_string(&res).expect("Failed to serialize to JSON");



    match json_string{
        json_string => Ok(json_string),
        _ => Err(Status::NotFound)
    }
    
}


#[post("/",  data = "<data>")]
async fn create_comment(pool: &State<Pool<Postgres>>,content_type: &ContentType , data: Data<'_>) -> Result<String, Status> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(
        vec![
            MultipartFormDataField::text("story_id"),
            MultipartFormDataField::text("username"),
            MultipartFormDataField::text("content")
            ]
        );
    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();


    let story_id = multipart_form_data.texts.remove("story_id").unwrap().remove(0).text.parse::<i32>().unwrap();
    let uname = multipart_form_data.texts.remove("username").unwrap().remove(0).text.into();
    let content = multipart_form_data.texts.remove("content").unwrap().remove(0).text.into();

    // let s = Story {username: uname.unwrap().remove(0).text.into(), content: content.unwrap().remove(0).text.into()};


    let comment: Result<(), sqlx::Error> = Comment::insert_comment_to_db(story_id,uname, content, &pool).await;


    match comment {
        Ok(_user) => Ok("Success".to_string()),
        _ => Err(Status::NotFound)
    }
    
    
}


#[delete("/",  data = "<data>")]
async fn delete_comments_by_user(pool: &State<Pool<Postgres>>,content_type: &ContentType , data: Data<'_>) -> Result<String, Status> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::text("username")
        
        ]);
    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();
    let username= multipart_form_data.texts.remove("username").unwrap().remove(0).text.into();

    // let user = User::insert_user_to_db(username,password, &pool).await;
    let res = Comment::delete_comments_by_user(username,&pool).await;




    match res {
        Ok(_user) => Ok("Success".to_string()),
        _ => Err(Status::NotFound)
    }
    
}

#[delete("/",  data = "<data>")]
async fn delete_comment(pool: &State<Pool<Postgres>>,content_type: &ContentType , data: Data<'_>) -> Result<String, Status> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::text("comment_id")
        
        ]);
    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();
    let comment_id= multipart_form_data.texts.remove("comment_id").unwrap().remove(0).text.parse::<i32>().unwrap();

    // let user = User::insert_user_to_db(username,password, &pool).await;
    let res = Comment::delete_comment(comment_id,&pool).await;




    match res {
        Ok(_user) => Ok("Success".to_string()),
        _ => Err(Status::NotFound)
    }
    
}

#[delete("/",  data = "<data>")]
async fn delete_all_story_comments(pool: &State<Pool<Postgres>>,content_type: &ContentType , data: Data<'_>) -> Result<String, Status> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::text("story_id")
        
        ]);
    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();
    let story_id= multipart_form_data.texts.remove("story_id").unwrap().remove(0).text.parse::<i32>().unwrap();

    // let user = User::insert_user_to_db(username,password, &pool).await;
    let res = Comment::delete_all_story_comments(story_id,&pool).await;




    match res {
        Ok(_user) => Ok("Success".to_string()),
        _ => Err(Status::NotFound)
    }
    
}


// END COMMENTS














#[launch]
async fn rocket() ->  _{
    dotenv::dotenv().expect("Unable to load environment variables from .env file");
  
    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    rocket::build()
    .mount("/", routes![index])
    .mount("/create_user", routes![create_user])
    .mount("/create_story", routes![create_story])
    .mount("/create_comment", routes![create_comment])
    .mount("/get_all_users", routes![get_all_users])
    .mount("/get_stories_by_user", routes![get_stories_by_user])
    .mount("/get_comments_by_story", routes![get_comments_by_story])
    .mount("/get_comments_by_user", routes![get_comments_by_user])
    .mount("/delete_comments_by_user", routes![delete_comments_by_user])
    .mount("/delete_comment", routes![delete_comment])
    .mount("/delete_all_story_comments", routes![delete_all_story_comments])
    .mount("/delete_story", routes![delete_story])
    .mount("/delete_stories_by_user", routes![delete_stories_by_user])
    .mount("/delete_user", routes![delete_user])
    .manage(pool)

}