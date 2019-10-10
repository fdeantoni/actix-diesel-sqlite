-- noinspection SqlNoDataSourceInspectionForFile

CREATE TABLE posts (
  id VARCHAR NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
)