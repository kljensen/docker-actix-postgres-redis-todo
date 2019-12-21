#!/bin/bash
echo -n "Running migrations..."
diesel database setup
echo -n "done."

env

if [[ -z "${DEVELOPMENT}" ]]; then
    echo "Running in production!"
    cargo run
else
    # Systemfd will help us move an open socket to the
    # new process when the process restarts. cargo-watch
    # will start a new process when our code changes.
    echo "Port = $PORT"
    systemfd --no-pid -s http::0.0.0.0:${PORT} -- cargo watch -x run
fi
