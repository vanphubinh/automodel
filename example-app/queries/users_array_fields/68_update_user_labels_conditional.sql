-- @automodel
--    description: Update user labels with conditional set (required jsonb[] column, no diff struct)
--    expect: exactly_one
--    types:
--      labels: "Vec<Option<crate::models::UserTag>>"
--      public.users.labels: "Vec<Option<crate::models::UserTag>>"
-- @end
UPDATE public.users
SET updated_at = NOW()
#[, name = #{name?}]
#[, labels = #{labels?}]
WHERE id = #{user_id}
RETURNING id, name, email, labels;
