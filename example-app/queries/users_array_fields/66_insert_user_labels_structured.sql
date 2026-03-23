-- @automodel
--    description: Insert user with labels using structured parameters (required jsonb[] column)
--    expect: exactly_one
--    parameters_type: true
--    types:
--      labels: "Vec<Option<crate::models::UserTag>>"
--      public.users.labels: "Vec<Option<crate::models::UserTag>>"
-- @end
INSERT INTO public.users (name, email, status, labels)
VALUES (#{name}, #{email}, 'pending', #{labels})
RETURNING id, name, email, labels;
