-- @automodel
--    description: Test batch insert with optional parameter (age is nullable)
--    expect: multiple
--    multiunzip: true
-- @end
INSERT INTO public.users (name, email, age)
SELECT name, email, age
FROM UNNEST(
        #{name}::text [],
        #{email}::text [],
        #{age?}::int4 []
    ) AS t(name, email, age)
RETURNING id, name, email, age, created_at
