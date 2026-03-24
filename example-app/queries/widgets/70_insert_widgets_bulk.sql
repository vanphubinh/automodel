-- @automodel
--    description: Bulk insert widgets using table composite type array with UNNEST
--    expect: multiple
-- @end

INSERT INTO public.widgets (name, weight, metadata)
SELECT r.name, r.weight, r.metadata
FROM UNNEST(#{items}::public.widgets[]) AS r(id, name, weight, metadata, created_at)
RETURNING id, name, weight, metadata
