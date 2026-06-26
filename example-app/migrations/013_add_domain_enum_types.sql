-- Domain type with CHECK (VALUE IN (...)) — should codegen as a Rust enum
CREATE DOMAIN product_priority AS TEXT CHECK (VALUE IN ('low', 'medium', 'high', 'urgent'));

ALTER TABLE public.products
    ADD COLUMN priority product_priority NOT NULL DEFAULT 'medium';
