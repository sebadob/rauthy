#!/bin/bash

export PGPASSWORD=123SuperSafe

psql -U postgres -d postgres -a -f /scripts/init.sql
psql -U postgres -d rauthy -a -f /scripts/init-rauthy.sql