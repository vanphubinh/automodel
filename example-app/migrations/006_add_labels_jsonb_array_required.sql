-- Add a required (NOT NULL) jsonb[] column for testing non-optional array of nullable JSONB elements
ALTER TABLE public.users ADD COLUMN labels jsonb[] NOT NULL DEFAULT '{}';
