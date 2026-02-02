-- @automodel
--    description: Test without multiunzip with @native suffix for Vec<Option<i32>>
--    types:
--      age: Vec<Option<i32>>@native
-- @end

INSERT INTO public.users (name, age)
SELECT * FROM UNNEST(
    #{names}::text[],
    #{age}::int4[]
)
RETURNING id, name, age;
