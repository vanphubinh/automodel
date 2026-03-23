-- @automodel
--    description: Update user labels (required jsonb[] column with nullable elements)
--    expect: exactly_one
--    types:
--      labels: "Vec<Option<crate::models::UserTag>>"
--      public.users.labels: "Vec<Option<crate::models::UserTag>>"
-- @end
UPDATE public.users
SET labels = #{labels}, updated_at = NOW()
WHERE id = #{user_id}
RETURNING id, name, email, labels;
