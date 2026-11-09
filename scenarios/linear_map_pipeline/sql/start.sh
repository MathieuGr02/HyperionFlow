#!/bin/bash

cd "$(dirname "$0")"

docker compose down --volumes
docker compose build
status=$?
if [ $status -neq 0 ]; then
    echo "Error building hyperion dockerfile"
    exit
fi

docker compose up --force-recreate
