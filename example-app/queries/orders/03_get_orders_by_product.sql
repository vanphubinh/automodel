-- @automodel
--    description: Get orders by product name (missing partition key filter — triggers partition pruning warning)
--    expect: multiple
--    ensure_indexes: true
-- @end

SELECT id, tenant_id, product_name, amount, created_at
FROM public.orders
WHERE product_name = #{product_name}
ORDER BY created_at DESC
