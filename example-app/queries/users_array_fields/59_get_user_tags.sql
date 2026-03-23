-- @automodel
--    description: Get user tags (jsonb[] column with nullable elements)
--    expect: exactly_one
--    types:
--      public.users.tags: "Vec<Option<crate::models::UserTag>>"
-- @end
SELECT id, name, email, tags
FROM public.users
WHERE id = #{user_id};
