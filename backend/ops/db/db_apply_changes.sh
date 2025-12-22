#!/bin/sh

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 ">>> Error: `sqlx` command is not available (not installed or its location is not in the PATH)."
  echo >&2 ">>> Hint: `sqlx-cli` needs to be installed."
  echo >&2 ">>>       To install it use:"
  echo >&2 ">>>     cargo install --version=0.8.6 sqlx-cli --no-default-features --features native-tls,postgres"
  echo >&2 ""
  exit 1
fi

BASEDIR=$(dirname $0)

sqlx migrate run --source $BASEDIR/migrations
