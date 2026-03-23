-- @automodel
--    description: Update user social links with conditional set (no diff struct)
--    expect: exactly_one
--    types:
--      social_links: "Vec<crate::models::UserSocialLink>"
--      public.users.social_links: "Vec<crate::models::UserSocialLink>"
-- @end
UPDATE public.users
SET updated_at = NOW()
#[, name = #{name?}]
#[, social_links = #{social_links?}]
WHERE id = #{user_id}
RETURNING id, name, email, social_links;
