-- @automodel
--    description: Insert a product with domain-typed fields
--    expect: exactly_one
-- @end

INSERT INTO public.products (name, price, contact_email)
VALUES (#{name}, #{price}, #{contact_email})
RETURNING id, name, price, contact_email
