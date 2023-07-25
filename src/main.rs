#[macro_use] extern crate rocket;

use core::result::Result;

use rocket::form::{Strict};
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, Repetition};

use rocket::Data;
use rocket::http::ContentType;

// #[derive(FromForm)]
// struct User {
//     username: Strict<String>,
//     uses_default: bool
// }


struct Story {
    username: Strict<String>,
    content: Strict<String>
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn insert_user_to_db(uname:String){
    println!("{}",uname);
 
}

#[post("/",  data = "<data>")]
async fn create_user(content_type: &ContentType , data: Data<'_>) -> &'static str {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![MultipartFormDataField::text("username")]);
    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();
    let username = multipart_form_data.texts.remove("username");

    

    if let Some(mut text_fields) = username {
        let text_field = text_fields.remove(0); // Because we only put one "text" field to the allowed_fields, the max length of this text_fields is 1.

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        // You can now deal with the text data.
        insert_user_to_db(_text);
        return "True";
    } else{
        return "False";
    };
    



}


#[post("/",  data = "<data>")]
async fn create_story(content_type: &ContentType , data: Data<'_>) -> &'static str {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(
        vec![
            MultipartFormDataField::text("username"),
            MultipartFormDataField::text("content"),
            ]
        );
    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();
    let uname = multipart_form_data.texts.remove("username");
    let content = multipart_form_data.texts.remove("content");

    let s = Story {username: uname.unwrap().remove(0).text.into(), content: content.unwrap().remove(0).text.into()};
// let p = Point { x: 0, y: 7 };
    
    // let mut s:Story;
    // if let Some(mut text_fields) = story {
    //     let text_field = text_fields.remove(0); // Because we only put one "text" field to the allowed_fields, the max length of this text_fields is 1.

    //     let _content_type = text_field.content_type;
    //     let _file_name = text_field.file_name;
    //     let _text = text_field.text;



    //     // You can now deal with the text data.
    //     // insert_user_to_db(_text);
    //     s.username = _text.clone().into();
        
    // }

    "True"
    
    
}


#[get("/")]
fn create_comment() -> &'static str {
    todo!();
}

#[get("/")]
fn test() -> &'static str {
    todo!();
}



#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/create_user", routes![create_user])
    .mount("/create_story", routes![create_story])
    .mount("/create_comment", routes![create_comment])
    .mount("/test", routes![test])
}