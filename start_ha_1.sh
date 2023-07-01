#!/bin/bash

HA_MODE='true' HA_HOSTS="http://$(hostname):8001,http://127.0.0.1:8002,http://127.0.0.1:8003" cargo run $1

