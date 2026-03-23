-- @automodel
--    description: Update user tags with conditional diff (jsonb[] column)
--    expect: exactly_one
--    conditions_type: true
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
