#!/bin/bash

server --dist-path=/usr/share/acm/dist --database-url=/acm/db.sqlite --port=80 --hostname=0.0.0.0 &
ramiel &

wait -n

exit $?
