use clap::{Arg, ArgAction, Command};
use config::{Config, ConfigError, Environment, File};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use log::debug;

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    id: String,
    fields: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct RecordsResponse {
    records: Vec<Record>,
}

#[derive(Debug, Deserialize)]
struct Settings {
    api_key: String,
    tables: HashMap<String, TableConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TableConfig {
    base_id: String,
    table_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Field {
    name: String,
    #[serde(rename = "type")]
    field_type: String,
}

#[derive(Debug, Deserialize)]
struct Table {
    name: String,
    fields: Vec<Field>,
}

#[derive(Debug, Deserialize)]
struct TablesResponse {
    tables: Vec<Table>,
}

impl Settings {
    fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(File::with_name("/Users/n/RustroverProjects/rau/config"))
            .add_source(Environment::with_prefix("AIRTABLE"))
            .build()?;
        settings.try_deserialize()
    }
}

async fn fetch_available_fields(api_key: &str, base_id: &str, table_name: &str) -> Result<Vec<Field>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://api.airtable.com/v0/meta/bases/{}/tables", base_id);
    let resp: TablesResponse = client
        .get(&url)
        .bearer_auth(api_key)
        .send()
        .await?
        .json()
        .await?;

    for table in resp.tables {
        if table_name == table.name {
            return Ok(table.fields);
        }
    }

    Ok(Vec::new())
}

async fn cache_available_fields(api_key: &str, base_id: &str, table_name: &str, cache_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let fields = fetch_available_fields(api_key, base_id, table_name).await?;
    let fields_json = serde_json::to_string(&fields)?;
    let mut file = fs::File::create(cache_file)?;
    file.write_all(fields_json.as_bytes())?;
    Ok(())
}

fn read_cached_fields(cache_file: &str) -> io::Result<Vec<Field>> {
    let fields_json = fs::read_to_string(cache_file)?;
    let fields: Vec<Field> = serde_json::from_str(&fields_json)?;
    Ok(fields)
}

// Helper function to parse JSON strings into JSON objects
fn parse_json_string(value: &str) -> serde_json::Value {
    serde_json::from_str(value).unwrap_or_else(|_| json!(value))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Settings::new()?;
    let api_key = &config.api_key;

    // Create CLI interface
    let matches = Command::new("Airtable CLI")
        .version("1.0")
        .about("Update or query Airtable records from the CLI")
        .arg(
            Arg::new("config")
                .help("The name of the configuration to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("record_id")
                .help("The ID of the record to update or query")
                .required(false)
                .index(2),
        )
        .arg(
            Arg::new("fields")
                .help("Fields to update in key=value format or fields to query for their values")
                .index(3)
                .num_args(1..)
                .require_equals(false)
                .required(false),
        )
        .arg(
            Arg::new("schema")
                .short('s')
                .long("schema")
                .action(ArgAction::SetTrue)
                .help("Output the schema")
                .conflicts_with("fields_flag"),
        )
        .arg(
            Arg::new("fields_flag")
                .short('f')
                .long("fields")
                .action(ArgAction::SetTrue)
                .help("Output the fields")
                .conflicts_with("schema"),
        )
        .arg(
            Arg::new("recent")
                .short('r')
                .long("recent")
                .action(ArgAction::SetTrue)
                .help("Output the 100 most recent record IDs and their names"),
        )
        .get_matches();

    let config_name = matches.get_one::<String>("config").expect("Configuration name is required");
    let record_id = matches.get_one::<String>("record_id");
    let fields: Vec<&str> = matches.get_many::<String>("fields").unwrap_or_default().map(|s| s.as_str()).collect();
    let output_schema = matches.get_flag("schema");
    let output_fields = matches.get_flag("fields_flag");
    let output_recent = matches.get_flag("recent");

    // Get the table configuration from the config
    let table_config = config.tables.get(config_name).expect("Configuration not found in config");

    // Cache available fields to a local file
    let cache_file = "available_fields_cache.json";
    cache_available_fields(&api_key, &table_config.base_id, &table_config.table_name, cache_file).await?;

    // Read available fields from cache
    let available_fields = read_cached_fields(cache_file)?;

    if output_schema {
        // Output the schema
        let schema_json = serde_json::to_string_pretty(&available_fields)?;
        println!("{}", schema_json);
        return Ok(());
    }

    // Filter out computed fields
    let updatable_fields: Vec<String> = available_fields
        .iter()
        .filter(|field| field.field_type != "computed" && field.field_type != "formula" && field.field_type != "rollup" && field.field_type != "lookup" && field.field_type != "lastModifiedTime" && field.field_type != "createdTime")
        .map(|field| field.name.clone())
        .collect();

    if output_fields {
        // Output the updatable fields
        let fields_json = serde_json::to_string_pretty(&updatable_fields)?;
        println!("{}", fields_json);
        return Ok(());
    }

    if output_recent {
        // Output the 100 most recent record IDs and their names
        let client = Client::new();
        let update_record_url = format!("https://api.airtable.com/v0/{}/{}", table_config.base_id, table_config.table_name);
        let query_url = format!("{}/?maxRecords=100&view=Grid%20view", update_record_url);

        // Make the API request
        let query_resp = client
            .get(&query_url)
            .bearer_auth(&api_key)
            .send()
            .await?;

        let status = query_resp.status();
        let text = query_resp.text().await?;

        if status.is_success() {
            let records_response: RecordsResponse = serde_json::from_str(&text)?;
            for record in records_response.records {
                let name = record.fields.get("Name").and_then(|v| v.as_str()).unwrap_or("<no name>").to_string();
                println!("ID: {}, Name: {}", record.id, name);
            }
        } else {
            eprintln!("Failed to query recent records. Status: {}, Response: {}", status, text);
        }

        return Ok(());
    }

    let client = Client::new();
    let update_record_url = format!("https://api.airtable.com/v0/{}/{}", table_config.base_id, table_config.table_name);

    if let Some(record_id) = record_id {
        if fields.is_empty() {
            // Query all fields for their values
            let query_record_url = format!("{}/{}", update_record_url, record_id);

            // Make the API request
            let query_resp = client
                .get(&query_record_url)
                .bearer_auth(&api_key)
                .send()
                .await?;

            let status = query_resp.status();
            let text = query_resp.text().await?;

            if status.is_success() {
                let record: Record = serde_json::from_str(&text)?;
                for (field, value) in record.fields.as_object().unwrap() {
                    println!("{}: {}", field, value);
                }
            } else {
                eprintln!("Failed to query record. Status: {}, Response: {}", status, text);
            }
        } else {
            // Check if fields are in key=value format or not
            let mut is_update = false;
            for field in &fields {
                if field.contains('=') {
                    is_update = true;
                    break;
                }
            }

            if is_update {
                // Parse fields into a JSON object
                let mut fields_json = Map::new();
                for field in fields {
                    let parts: Vec<&str> = field.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        fields_json.insert(parts[0].to_string(), parse_json_string(parts[1]));
                    } else {
                        eprintln!("Invalid field format: {}", field);
                        return Ok(());
                    }
                }

                // Update existing record
                let update_data = json!({
                    "records": [
                        {
                            "id": record_id,
                            "fields": Value::Object(fields_json)
                        }
                    ]
                });

                // Make the API request
                let update_resp = client
                    .patch(&update_record_url)
                    .bearer_auth(&api_key)
                    .header("Content-Type", "application/json")
                    .json(&update_data)
                    .send()
                    .await?;

                let status = update_resp.status();
                let text = update_resp.text().await?;

                if status.is_success() {
                    let updated_records: RecordsResponse = serde_json::from_str(&text)?;
                    println!("Updated Record");
                } else {
                    eprintln!("Failed to update record. Status: {}, Response: {}", status, text);
                }
            } else {
                // Query specific fields for their values
                let query_record_url = format!("{}/{}", update_record_url, record_id);

                // Make the API request
                let query_resp = client
                    .get(&query_record_url)
                    .bearer_auth(&api_key)
                    .send()
                    .await?;

                let status = query_resp.status();
                let text = query_resp.text().await?;

                if status.is_success() {
                    let record: Record = serde_json::from_str(&text)?;
                    for field in fields {
                        if let Some(value) = record.fields.get(field) {
                            println!("{}: {}", field, value);
                        } else {
                            println!("{}: <no value>", field);
                        }
                    }
                } else {
                    eprintln!("Failed to query record. Status: {}, Response: {}", status, text);
                }
            }
        }
    } else {
        // Create new record with empty structure
        let empty_fields: Map<String, Value> = updatable_fields.into_iter().map(|f| (f, json!(null))).collect();
        let create_data = json!({
            "records": [
                {
                    "fields": Value::Object(empty_fields)
                }
            ]
        });

        // Make the API request
        let create_resp = client
            .post(&update_record_url)
            .bearer_auth(&api_key)
            .header("Content-Type", "application/json")
            .json(&create_data)
            .send()
            .await?;

        let status = create_resp.status();
        let text = create_resp.text().await?;

        if status.is_success() {
            let created_records: RecordsResponse = serde_json::from_str(&text)?;
            if let Some(new_record) = created_records.records.get(0) {
                eprintln!("Created Record ID");
                println!("{}", new_record.id);
            } else {
                eprintln!("Failed to parse the response after creating a new record.");
            }
        } else {
            eprintln!("Failed to create record. Status: {}, Response: {}", status, text);
        }
    }

    Ok(())
}

fn main_autocomplete() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let config = Settings::new().unwrap();

    // Create CLI interface for autocomplete
    let matches = Command::new("Airtable CLI Autocomplete")
        .version("1.0")
        .about("Autocomplete for Airtable CLI")
        .arg(
            Arg::new("config")
                .help("The name of the configuration to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let config_name = matches.get_one::<String>("config").expect("Configuration name is required");

    // Get the table configuration from the config
    let table_config = config.tables.get(config_name).expect("Configuration not found in config");

    rt.block_on(cache_available_fields(&config.api_key, &table_config.base_id, &table_config.table_name, "available_fields_cache.json")).unwrap();

    let available_fields = read_cached_fields("available_fields_cache.json").unwrap();

    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let input: Vec<&str> = line.split('=').collect();
                if input.len() == 2 {
                    let field = input[0];
                    if available_fields.iter().any(|f| f.name == field) {
                        println!("Field: {} Value: {}", field, input[1]);
                    } else {
                        println!("Unknown field: {}", field);
                    }
                } else {
                    println!("Invalid input. Use format key=value");
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
