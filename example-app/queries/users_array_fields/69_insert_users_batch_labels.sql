-- @automodel
--    description: Batch insert users with labels using multiunzip (required jsonb[] column)
--    expect: multiple
--    multiunzip: true
--    types:
--      labels: "Vec<Option<crate::models::UserTag>>"
--      public.users.labels: "Vec<Option<crate::models::UserTag>>"
-- @end
INSERT INTO public.users (name, email, labels)
SELECT name, email,
    ARRAY(SELECT jsonb_array_elements(labels))
FROM UNNEST(
        #{name}::text [],
        #{email}::text [],
        #{labels}::jsonb []
    ) AS t(name, email, labels)
RETURNING id, name, email, labels;
