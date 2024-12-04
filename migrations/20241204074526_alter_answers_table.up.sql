-- Add up migration script here
ALTER TABLE answers
    RENAME COLUMN corresponding_question TO question_id;