#!/usr/bin/env bash
set -x
set -eo pipefail

#check if a custom user  has been set,otherwise use default user
DB_USER=${POSTGRES_USER:=postgres}
#check if a custom password  has been set, otherwise uses default password
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
#check if a  custom database has been set, otherwise uses default database
DB_NAME="${POSTGRES_DB:=newsletter}"
#check if a custom port has been set, otherwise used default port
DB_PORT="${POSTGRES_PORT:=5432}"


#Launch postgres using docker
docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
postgres -N 1000

