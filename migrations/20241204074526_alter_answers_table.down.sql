-- Add down migration script here
ALTER TABLE answers
    RENAME COLUMN question_id TO corresponding_question;