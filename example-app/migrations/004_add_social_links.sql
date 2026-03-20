-- Add social_links JSONB column to users table
ALTER TABLE public.users ADD COLUMN social_links JSONB DEFAULT '[]'::jsonb;

-- Update existing users with empty array
UPDATE public.users SET social_links = '[]'::jsonb WHERE social_links IS NULL;

-- Add a sample social link to test data
UPDATE public.users 
SET social_links = '[
    {"name": "GitHub", "url": "https://github.com/johndoe"},
    {"name": "LinkedIn", "url": "https://linkedin.com/in/johndoe"}
]'::jsonb
WHERE email = 'john@example.com';

UPDATE public.users 
SET social_links = '[
    {"name": "Twitter", "url": "https://twitter.com/janesmith"}
]'::jsonb
WHERE email = 'jane@example.com';
