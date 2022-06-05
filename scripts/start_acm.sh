#!/bin/bash

server --database-url=/acm/db.sqlite --port=80 --hostname=0.0.0.0 &
ramiel &

wait -n

exit $?
