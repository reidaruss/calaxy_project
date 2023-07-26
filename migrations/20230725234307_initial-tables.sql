-- Add migration script here
CREATE TABLE "users" (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    pass VARCHAR(255) NOT NULL
);

CREATE TABLE "story" (
    story_id SERIAL PRIMARY KEY,
    username VARCHAR(255) REFERENCES users (username) NOT NULL,
    story_content TEXT NOT NULL
);

CREATE TABLE "comment" (
    comment_id SERIAL PRIMARY KEY,
    story_id INTEGER REFERENCES "story" (story_id) NOT NULL,
    username VARCHAR(255) REFERENCES "users" (username) NOT NULL,
    comment_content TEXT NOT NULL
);