#!/bin/bash

HA_MODE=true HA_HOSTS="http://127.0.0.1:8001,http://127.0.0.1:8002,http://$(hostname):8003" LISTEN_ADDRESS=0.0.0.0:9091 HOST=localhost:9091 cargo run

