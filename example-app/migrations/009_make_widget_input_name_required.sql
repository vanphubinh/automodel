-- Make widget_input.name non-nullable via pg_attribute catalog update.
-- PostgreSQL doesn't support NOT NULL on composite type attributes via DDL,
-- so we update the catalog directly.
UPDATE pg_attribute
SET attnotnull = true
WHERE attrelid = (SELECT typrelid FROM pg_type WHERE typname = 'widget_input')
  AND attname = 'name';
