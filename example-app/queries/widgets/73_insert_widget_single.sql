-- @automodel
--    description: Insert a single widget using a singular composite type parameter
--    expect: exactly_one
-- @end

INSERT INTO public.widgets (name, weight, metadata)
SELECT r.name, r.weight, r.metadata
FROM (SELECT (#{item}::public.widget_input).*) AS r
RETURNING id, name, weight, metadata
