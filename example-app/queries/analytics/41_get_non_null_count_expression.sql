-- @automodel
--    description: Test non-null override with native {col!} syntax on count expression
--    expect: exactly_one
-- @end

SELECT count(*) + count(*) AS {total!} FROM public.users
