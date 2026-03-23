-- Add a jsonb[] column for testing array of nullable JSONB elements
ALTER TABLE public.users ADD COLUMN tags jsonb[] DEFAULT NULL;
