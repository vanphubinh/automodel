-- @automodel
--    description: Test non-null override with native {col!} syntax on boolean literal in RETURNING
--    expect: possible_one
-- @end

UPDATE public.users
SET name = #{name}
WHERE id = #{id}
RETURNING true AS {applied!}
