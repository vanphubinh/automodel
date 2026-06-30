-- @automodel
--    description: Insert a product with domain-typed fields
--    expect: exactly_one
--    types:
--       public.positive_int: std::num::NonZeroI32
-- @end

INSERT INTO public.products (name, price, contact_email)
VALUES (#{name}, #{price}, #{contact_email})
RETURNING id, name, price, contact_email
