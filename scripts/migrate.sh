#!/usr/bin/env bash
set -euo pipefail

DB_URL="${AUTOMODEL_DATABASE_URL:?AUTOMODEL_DATABASE_URL must be set}"

for f in example-app/migrations/*.sql; do
  echo "=== Running $f ==="
  psql "$DB_URL" -f "$f"
done

echo "=== All migrations applied ==="
