-- @automodel
--    description: Update user tags with conditional set (jsonb[] column, no diff struct)
--    expect: exactly_one
--    types:
--      tags: "Vec<Option<crate::models::UserTag>>"
--      public.users.tags: "Vec<Option<crate::models::UserTag>>"
-- @end
UPDATE public.users
SET updated_at = NOW()
#[, name = #{name?}]
#[, tags = #{tags?}]
WHERE id = #{user_id}
RETURNING id, name, email, tags;
