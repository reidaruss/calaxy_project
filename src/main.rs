#[macro_use] extern crate rocket;

use core::result::Result;

use rocket::form::{Strict};
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, Repetition};
use rocket::State;

use serde::{Serialize, Deserialize};

use rocket::Data;
use rocket::http::ContentType;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, outcome};




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

        let insert = sqlx::query_as!(
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
}

impl Story {
    async fn insert_story_to_db(username: String, content:String, pool: &Pool<Postgres>) -> Result<(),sqlx::Error> {

        let insert = sqlx::query_as!(
            Story,
            "insert into story (username, story_content) values ($1, $2)",
            username.clone(),
            content
        ).execute(pool)
        .await.expect("Unable to insert story");



        Ok(())
    }
    async fn get_stories_by_user(username: String, pool: &Pool<Postgres>) -> Vec<Story> {
        // let q = format!("select * from story where username = {}", username);

        let stories: Vec<Story> = sqlx::query_as!(
            Story,
            "select * from story where username = $1",
            username.clone()
        ).fetch_all(pool)
        .await.expect("Unable to get stories by user");



        stories
    }
}


impl Comment {
    async fn insert_comment_to_db(story_id:i32,username: String, content:String, pool: &Pool<Postgres>) -> Result<(),sqlx::Error> {

        let insert = sqlx::query_as!(
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
        // let q = format!("select * from story where username = {}", username);

        let comments: Vec<Comment> = sqlx::query_as!(
            Comment,
            "select * from comment where story_id = $1",
            story_id
        ).fetch_all(pool)
        .await.expect("Unable to get stories by comment");



        comments
    }

    async fn get_comments_by_user(username: String, pool: &Pool<Postgres>) -> Vec<Comment> {
        // let q = format!("select * from story where username = {}", username);

        let comments: Vec<Comment> = sqlx::query_as!(
            Comment,
            "select * from comment where username = $1",
            username
        ).fetch_all(pool)
        .await.expect("Unable to get stories by comment");



        comments
    }

    async fn delete_comments_by_user(username: String, pool: &Pool<Postgres>) -> Result<(),sqlx::Error> {
        // let q = format!("select * from story where username = {}", username);

        let insert = sqlx::query_as!(
            Comment,
            "DELETE FROM comment WHERE username = $1",
            username
        ).execute(pool)
        .await.expect("Unable to delete comments by user");



        Ok(())
    }
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// fn insert_user_to_db(uname:String){
//     println!("{}",uname);
 
// }

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

// async fn get_all_users(pool: &State<Pool<Postgres>>) -> Result<String, Status> {

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
    let res: Vec<Comment> = Comment::delete_comments_by_user(username,&pool).await;




    match res {
        Ok(_user) => Ok("Success".to_string()),
        _ => Err(Status::NotFound)
    }
    
}
// END COMMENTS


// #[get("/")]
// fn create_comment() -> &'static str {
//     todo!();
// }




use sqlx::PgPool;






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
    .manage(pool)

}