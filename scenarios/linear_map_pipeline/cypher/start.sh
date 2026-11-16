#!/bin/bash

cd "$(dirname "$0")"

docker compose down --volumes

docker compose build
status=$?

if [ $status -ne 0 ]; then
    echo "Error building hyperion dockerfile"
    exit 1
fi

docker compose up --force-recreate
