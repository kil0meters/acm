#!/bin/bash

server --dist-path=/usr/share/acm/dist --database-url=/acm/db.sqlite &
ramiel &

wait -n

exit $?
