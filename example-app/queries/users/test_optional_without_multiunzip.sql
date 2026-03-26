-- @automodel
--    description: Test batch insert with nullable array elements (not entire array optional)
--    expect: multiple
-- @end
INSERT INTO public.users (name, email, age)
SELECT name, email, age
FROM UNNEST(
        #{name}::text [],
        #{email}::text [],
        #{age[?]}::int4 []
    ) AS t(name, email, age)
RETURNING id, name, email, age, created_at
