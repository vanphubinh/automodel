-- Composite type for bulk-inserting users with social links via UNNEST
CREATE TYPE public.user_with_links_input AS (
    name TEXT,
    email TEXT,
    social_links JSONB
);
