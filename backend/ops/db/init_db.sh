#!/bin/sh

docker compose up -d

# Wait for the database to be ready
until docker compose exec postgres pg_isready -U postgres; do
  echo "Waiting for the database to be ready..."
  sleep 2
done

echo "The database is ready!"
