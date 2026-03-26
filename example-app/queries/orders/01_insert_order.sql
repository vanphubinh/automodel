-- @automodel
--    description: Insert a new order
--    expect: exactly_one
-- @end

INSERT INTO public.orders (tenant_id, product_name, amount)
VALUES (#{tenant_id}, #{product_name}, #{amount})
RETURNING id, tenant_id, product_name, amount, created_at
