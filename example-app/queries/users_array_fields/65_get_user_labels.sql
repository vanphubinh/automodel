-- @automodel
--    description: Get user labels (required jsonb[] column with nullable elements)
--    expect: exactly_one
--    types:
--      public.users.labels: "Vec<Option<crate::models::UserTag>>"
-- @end
SELECT id, name, email, labels
FROM public.users
WHERE id = #{user_id};
