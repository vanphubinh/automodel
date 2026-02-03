-- @automodel
--    description: Test returning table row type as nested data
-- @end

SELECT 
    u.id,
    u.name,
    u as user_details
FROM public.users u
WHERE u.id = #{user_id};
