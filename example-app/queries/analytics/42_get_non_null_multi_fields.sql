-- @automodel
--    description: Test non-null override on multiple expression columns in one query
--    expect: exactly_one
-- @end

SELECT
    count(*) AS {user_count!},
    count(*) + count(*) AS {double_count!},
    true AS {is_valid!},
    now() AS "current_time!",
    'hello' AS {greeting!}
FROM public.users
