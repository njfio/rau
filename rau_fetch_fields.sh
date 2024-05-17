#!/bin/bash
config_name="$1"

# Fetch fields based on config_name and process the output
fields_json=$(rau "$config_name" -f)

# Parse JSON and extract field names
field_names=$(echo "$fields_json" | jq -r '.[]')

# Loop through each field name and add quotes
echo "$field_names" | while IFS= read -r field; do
  echo $field
done
