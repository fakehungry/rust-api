#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version='~0.8' sqlx-cli --no-default-features --features rustls,postgres"
  echo >&2 "to install it."
  exit 1
fi

DB_PORT="${DB_PORT:=5432}"
SUPERUSER="${SUPERUSER:=postgres}"
SUPERUSER_PWD="${SUPERUSER_PWD:=password}"
APP_USER="${APP_USER:=hean}"
APP_USER_PWD="${APP_USER_PWD:=heankub22}"
DB_NAME="${DB_NAME:=rust_db}"

if [[ -z "${SKIP_DOCKER}" ]]; then
  RUNNING_POSTGRES_CONTAINER=$(docker ps --filter 'name=rust_db' --format '{{.ID}}')
  if [[ -n $RUNNING_POSTGRES_CONTAINER ]]; then
    echo >&2 "there is a rust_db container already running, kill it with"
    echo >&2 "    docker kill ${RUNNING_POSTGRES_CONTAINER}"
    exit 1
  fi
  CONTAINER_NAME="rust_db"
  docker run \
    --env POSTGRES_USER=${SUPERUSER} \
    --env POSTGRES_PASSWORD=${SUPERUSER_PWD} \
    --health-cmd="pg_isready -U ${SUPERUSER} || exit 1" \
    --health-interval=1s \
    --health-timeout=5s \
    --health-retries=5 \
    --publish "${DB_PORT}":5432 \
    --detach \
    --name ${CONTAINER_NAME} \
    postgres -N 1000 # max connections

  until [ \
    "$(docker inspect -f "{{.State.Health.Status}}" ${CONTAINER_NAME})" == \
    "healthy" \
  ]; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
  done

  # Create the application user
  CREATE_QUERY="CREATE USER ${APP_USER} WITH PASSWORD '${APP_USER_PWD}';"
  docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${CREATE_QUERY}"

  # Grant the application user permission to create databases
  GRANT_QUERY="ALTER USER ${APP_USER} CREATEDB;"
  docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${GRANT_QUERY}"
fi

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

export DATABASE_URL=postgres://${APP_USER}:${APP_USER_PWD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&2 echo "Database has been created and migrated. Ready to go!"