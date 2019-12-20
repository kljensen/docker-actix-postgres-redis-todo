#!/bin/sh
diesel database setup
echo "done with setup!"
cargo run
# sleep 1000