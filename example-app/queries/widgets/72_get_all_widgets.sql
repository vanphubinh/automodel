-- @automodel
--    description: Get all widgets
--    expect: multiple
-- @end

SELECT id, name, weight, metadata, created_at FROM public.widgets ORDER BY id
