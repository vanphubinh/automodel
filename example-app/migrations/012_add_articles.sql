-- Add articles table with nullable JSONB columns for testing multiunzip
CREATE TABLE public.articles (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    metadata JSONB DEFAULT NULL,
    contributors JSONB DEFAULT NULL
);
