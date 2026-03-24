-- @automodel
--    description: Bulk insert users with social links using composite type UNNEST
--    expect: multiple
--    types:
--      public.users.social_links: "Vec<crate::models::UserSocialLink>"
-- @end

INSERT INTO public.users (name, email, social_links)
SELECT r.name, r.email, r.social_links
FROM UNNEST(#{items}::public.user_with_links_input[]) AS r(name, email, social_links)
RETURNING id, name, email, social_links
