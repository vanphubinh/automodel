-- @automodel
--    description: Test non-null override with sqlx-compatible "col!" syntax on comparison expression
--    expect: exactly_one
-- @end

SELECT created_at > now() - interval '1 year' AS "is_recent!" FROM public.users WHERE id = #{id}
