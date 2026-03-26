-- Domain types for testing alias codegen
CREATE DOMAIN positive_int AS INTEGER CHECK (VALUE > 0);
CREATE DOMAIN email_address AS VARCHAR(255) CHECK (VALUE ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$');

-- Table using domain types
CREATE TABLE IF NOT EXISTS public.products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    price positive_int NOT NULL,
    contact_email email_address NOT NULL
);

INSERT INTO public.products (name, price, contact_email)
VALUES ('Widget', 100, 'sales@example.com');
