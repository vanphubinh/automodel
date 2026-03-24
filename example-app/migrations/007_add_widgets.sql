-- Schema for testing composite type input parameters
-- Both table-based composite types and custom composite types

-- Nested composite type used inside other composites
CREATE TYPE public.widget_metadata AS (
    color TEXT,
    version INT4
);

CREATE TABLE IF NOT EXISTS public.widgets (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    weight FLOAT8,
    metadata public.widget_metadata,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Custom composite type for bulk input (decoupled from table structure)
CREATE TYPE public.widget_input AS (
    name TEXT,
    weight FLOAT8,
    metadata public.widget_metadata
);

-- Insert some sample widgets
INSERT INTO public.widgets (name, weight, metadata) VALUES
    ('Sprocket', 1.5, ROW('red', 1)::public.widget_metadata),
    ('Gear', 2.3, ROW('blue', 2)::public.widget_metadata),
    ('Bolt', 0.1, NULL)
ON CONFLICT DO NOTHING;
