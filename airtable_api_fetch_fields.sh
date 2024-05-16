#!/bin/bash
config_name="$1"

# Fetch fields based on config_name and process the output
fields_json=$(/Users/n/RustroverProjects/rau/target/release/rau "$config_name" -f)

# Parse JSON and extract field names
field_names=$(echo "$fields_json" | jq -r '.[]')

for field in $field_names; do
  echo "$field"
done
