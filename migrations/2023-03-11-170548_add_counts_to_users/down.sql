-- This file should undo anything in `up.sql`
ALTER TABLE users
    DROP followers_count,
    DROP following_count;
