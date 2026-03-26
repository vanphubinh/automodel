-- @automodel
--    description: Get orders for a specific tenant (partition-pruned via equality on partition key)
--    expect: multiple
--    ensure_indexes: true
-- @end

SELECT id, tenant_id, product_name, amount, created_at
FROM public.orders
WHERE tenant_id = #{tenant_id}
ORDER BY created_at DESC
