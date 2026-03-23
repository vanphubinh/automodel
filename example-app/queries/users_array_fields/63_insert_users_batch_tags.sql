-- @automodel
--    description: Batch insert users with tags using multiunzip (jsonb[] column)
--    expect: multiple
--    multiunzip: true
--    types:
--      tags: "Vec<Option<crate::models::UserTag>>"
--      public.users.tags: "Vec<Option<crate::models::UserTag>>"
-- @end
INSERT INTO public.users (name, email, tags)
SELECT name, email,
    CASE WHEN tags IS NULL THEN NULL
    ELSE ARRAY(SELECT jsonb_array_elements(tags)) END
FROM UNNEST(
        #{name}::text [],
        #{email}::text [],
        #{tags}::jsonb []
    ) AS t(name, email, tags)
RETURNING id, name, email, tags;


