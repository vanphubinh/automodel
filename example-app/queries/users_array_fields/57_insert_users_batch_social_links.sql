-- @automodel
--    description: Batch insert users with optional social links using multiunzip
--    expect: multiple
--    multiunzip: true
--    types:
--      social_links: "Vec<crate::models::UserSocialLink>"
--      public.users.social_links: "Vec<crate::models::UserSocialLink>"
-- @end
INSERT INTO public.users (name, email, social_links)
SELECT name, email, social_links
FROM UNNEST(
        #{name}::text [],
        #{email}::text [],
        #{social_links?}::jsonb []
    ) AS t(name, email, social_links)
RETURNING id, name, email, social_links;
