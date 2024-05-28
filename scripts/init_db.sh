#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error ! psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 " Sqlx is not yet installed."
  echo >&2 "use : "
  echo >&2 "cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
  echo >&2 "to install it"
  exit 1
fi

#check if a custom user  has been set,otherwise use default user
DB_USER=${POSTGRES_USER:=postgres}
#check if a custom password  has been set, otherwise uses default password
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
#check if a  custom database has been set, otherwise uses default database
DB_NAME="${POSTGRES_DB:=newsletter}"
#check if a custom port has been set, otherwise used default port
DB_PORT="${POSTGRES_PORT:=5432}"


#Launch postgres using docker
#allow to skip docker if alreadyb postrges running
if [[ -z "${SKIP_DOCKER}" ]]
then
docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
postgres -N 1000
fi

#keep pinging postgres until its ready to accept commands
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q' ; do
  >&2 echo " Postgres is unavailable at this moment . please try later"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!  "

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run
>&2 echo " Postgres has been migrated . Ready to  go!"