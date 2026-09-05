#!/usr/bin/env bash
# Applies every migration in this directory, in filename order, and records
# what it applied in iso20022.schema_migrations.
#
# Every migration in this directory is idempotent and wrapped in a single
# transaction, so re-running this script is safe: a file that is already
# applied is a no-op, and a file that fails rolls back rather than leaving the
# schema half-migrated. The tracking table therefore exists for visibility -
# "what is applied here, and when" - rather than to make re-runs safe.
#
# Connection settings come from the standard libpq environment variables
# (PGHOST, PGPORT, PGUSER, PGPASSWORD, PGDATABASE) or from a single argument:
#
#   ./apply.sh                          # uses PG* environment variables
#   ./apply.sh "postgresql://user@host/db"
#
set -euo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONN="${1:-}"
psql_run() { if [ -n "$CONN" ]; then psql "$CONN" "$@"; else psql "$@"; fi; }

psql_run -v ON_ERROR_STOP=1 -q <<'SQL'
CREATE SCHEMA IF NOT EXISTS iso20022;
CREATE TABLE IF NOT EXISTS iso20022.schema_migrations (
    filename    text        PRIMARY KEY,
    checksum    text        NOT NULL,
    applied_at  timestamptz NOT NULL DEFAULT now()
);
SQL

applied=0
skipped=0

for f in $(ls "$DIR"/[0-9]*.sql | sort); do
    name="$(basename "$f")"
    sum="$(sha256sum "$f" | cut -c1-64)"

    recorded="$(psql_run -tAc \
        "SELECT checksum FROM iso20022.schema_migrations WHERE filename = '$name'")"

    if [ "$recorded" = "$sum" ]; then
        skipped=$((skipped + 1))
        continue
    fi

    if [ -n "$recorded" ]; then
        echo "note: $name has changed since it was applied; re-applying (it is idempotent)"
    fi

    echo "applying $name"
    psql_run -v ON_ERROR_STOP=1 -q -f "$f"
    psql_run -v ON_ERROR_STOP=1 -q -c \
        "INSERT INTO iso20022.schema_migrations (filename, checksum) VALUES ('$name', '$sum')
         ON CONFLICT (filename) DO UPDATE SET checksum = EXCLUDED.checksum, applied_at = now()"
    applied=$((applied + 1))
done

echo "done: $applied applied, $skipped already up to date"
