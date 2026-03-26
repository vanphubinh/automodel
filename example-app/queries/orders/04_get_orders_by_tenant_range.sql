-- @automodel
--    description: Get orders for a tenant range (range filter on hash partition key — no pruning)
--    expect: multiple
--    ensure_indexes: true
-- @end

SELECT id, tenant_id, product_name, amount, created_at
FROM public.orders
WHERE tenant_id > #{min_tenant_id}
ORDER BY created_at DESC
