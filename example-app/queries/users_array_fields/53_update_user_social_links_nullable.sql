-- @automodel
--    description: Update user's social links with nullable value
--    expect: exactly_one
--    types:
--      social_links: "Vec<crate::models::UserSocialLink>"
--      public.users.social_links: "Vec<crate::models::UserSocialLink>"
-- @end
UPDATE public.users
SET social_links = #{social_links?}, updated_at = NOW()
WHERE id = #{user_id}
RETURNING id, name, email, social_links;
