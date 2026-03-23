-- @automodel
--    description: Insert user with tags using structured parameters (jsonb[] column)
--    expect: exactly_one
--    parameters_type: true
--    types:
--      tags: "Vec<Option<crate::models::UserTag>>"
--      public.users.tags: "Vec<Option<crate::models::UserTag>>"
-- @end
INSERT INTO public.users (name, email, status, tags)
VALUES (#{name}, #{email}, 'pending', #{tags})
RETURNING id, name, email, tags;
