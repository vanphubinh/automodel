-- Hash-partitioned orders table for partition pruning tests
CREATE TABLE IF NOT EXISTS public.orders (
    id SERIAL,
    tenant_id INTEGER NOT NULL,
    product_name TEXT NOT NULL,
    amount NUMERIC(10, 2) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (id, tenant_id)
) PARTITION BY HASH (tenant_id);

CREATE TABLE IF NOT EXISTS public.orders_p0 PARTITION OF public.orders
    FOR VALUES WITH (MODULUS 4, REMAINDER 0);
CREATE TABLE IF NOT EXISTS public.orders_p1 PARTITION OF public.orders
    FOR VALUES WITH (MODULUS 4, REMAINDER 1);
CREATE TABLE IF NOT EXISTS public.orders_p2 PARTITION OF public.orders
    FOR VALUES WITH (MODULUS 4, REMAINDER 2);
CREATE TABLE IF NOT EXISTS public.orders_p3 PARTITION OF public.orders
    FOR VALUES WITH (MODULUS 4, REMAINDER 3);
