-- @automodel
--    description: Get user with their social links
--    expect: exactly_one
--    types:
--      public.users.social_links: "Vec<crate::models::UserSocialLink>"
-- @end
SELECT id, name, email, social_links, created_at
FROM public.users
WHERE id = #{user_id};
