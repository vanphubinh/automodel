-- @automodel
--    description: Add a new user with social links
--    expect: exactly_one
--    types:
--      social_links: "Vec<crate::models::UserSocialLink>"
--      public.users.social_links: "Vec<crate::models::UserSocialLink>"
-- @end
INSERT INTO public.users (name, email, status, social_links)
VALUES (#{name}, #{email}, 'pending', #{social_links})
RETURNING id, name, email, social_links;
