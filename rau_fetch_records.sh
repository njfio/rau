#!/bin/bash
config_name="$1"
rau "$config_name" -r | awk -F', ' '{print $1 "," $2}' | sed 's/ID: //; s/Name: //'
