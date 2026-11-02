#!/bin/bash

cd "$(dirname "$0")"

docker compose down --volumes
docker compose build --no-cache
docker compose up --force-recreate
