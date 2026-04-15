-- @automodel
--    description: Test non-null override on multiple expression columns returning multiple rows
--    expect: multiple
-- @end

SELECT
    id AS {user_id!},
    name AS {user_name!},
    created_at > now() - interval '1 year' AS {is_recent!},
    true AS "is_active!"
FROM public.users
