-- @automodel
--    description: Update user tags (jsonb[] column with nullable elements)
--    expect: exactly_one
--    types:
--      tags: "Vec<Option<crate::models::UserTag>>"
--      public.users.tags: "Vec<Option<crate::models::UserTag>>"
-- @end
UPDATE public.users
SET tags = #{tags}, updated_at = NOW()
WHERE id = #{user_id}
RETURNING id, name, email, tags;
