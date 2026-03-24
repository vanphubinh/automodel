-- @automodel
--    description: Bulk insert widgets using custom composite type array with UNNEST
--    expect: multiple
-- @end

INSERT INTO public.widgets (name, weight, metadata)
SELECT r.name, r.weight, r.metadata
FROM UNNEST(#{items}::public.widget_input[]) AS r(name, weight, metadata)
RETURNING id, name, weight, metadata
