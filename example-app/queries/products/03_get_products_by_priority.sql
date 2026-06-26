-- @automodel
--    description: Get products filtered by priority domain enum
--    expect: multiple
-- @end

SELECT id, name, price, contact_email, priority
FROM public.products
WHERE priority = #{priority}
