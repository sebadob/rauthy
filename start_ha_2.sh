#!/bin/bash

HA_MODE=true HA_HOSTS="http://127.0.0.1:8001,http://$(hostname):8002,http://127.0.0.1:8003" LISTEN_ADDRESS=0.0.0.0:9090 HOST=localhost:9090 cargo run

