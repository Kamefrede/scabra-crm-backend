-- Your SQL goes here
CREATE TABLE articles (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  body TEXT NOT NULL,
  published_at TIMESTAMP WITH TIME ZONE NOT NULL,
  author_id INTEGER REFERENCES users(id) NOT NULL
)