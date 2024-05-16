```markdown
# Rust Airtable Utility (RAU)

A command-line utility written in Rust to interact with the Airtable API, enabling you to query, create, and update records.

## Features

- **Query Records:** Retrieve records and their field values by record ID or query parameters.
- **Create Records:** Generate new records with specified field values.
- **Update Records:** Modify existing records by updating field values.
- **Cache Management:** Locally cache available fields for faster subsequent requests.
- **Schema and Fields Output:** Display the table schema or available fields for reference.
- **Recent Records:** List the 100 most recent record IDs and their names for quick access.
- **Configuration:** Easily manage multiple Airtable bases and tables through a `config.toml` file.
- **Environment Variables:** Securely store your API key using environment variables.

## Installation

1. **Install Rust:** If you don't have Rust installed, download and install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
2. **Clone the Repository:**
   ```bash
   git clone https://github.com/yourusername/rau.git
   ```
3. **Navigate to the Project Directory:**
   ```bash
   cd rau
   ```
4. **Build the Project:**
   ```bash
   cargo build
   ```

## Configuration

1. **Create `config.toml`:** Place a `config.toml` file in the project root directory.
2. **Define Airtable Configurations:**
   ```toml
   [tables]
   tweets = { base_id = "appEo7LBNoYQRwEc0", table_name = "Table1" }
   pokemons = { base_id = "app2jJgrXCQirseg5", table_name = "Pokemon" }
   prompts = { base_id = "appzdA0NkqZ7JYMeP", table_name = "Prompt PreSet" }
   ```
3. **Set API Key:** You can either directly add your API key to the `config.toml` or use an environment variable.
   - **Directly in `config.toml`:**
     ```toml
     api_key = "YOUR_AIRTABLE_API_KEY"
     ```
   - **Environment Variable:**
     ```bash
     export AIRTABLE_API_KEY="YOUR_AIRTABLE_API_KEY"
     ```

## Usage

```bash
rau <config_name> [record_id] [fields] [options]
```

**Arguments:**

- `<config_name>`: The name of the configuration in your `config.toml` file.
- `[record_id]` (optional): The ID of the record to interact with.
- `[fields]` (optional): A list of fields to update or query, in key=value format.

**Options:**

- `-s, --schema`: Output the table schema.
- `-f, --fields`: Output the available fields for the table.
- `-r, --recent`: Output the 100 most recent record IDs and their names.

**Examples:**

- **Query all fields of a record:**
  ```bash
  rau tweets rec123
  ```
- **Query specific fields of a record:**
  ```bash
  rau tweets rec123 Name Content
  ```
- **Update a record:**
  ```bash
  rau tweets rec123 Name="Updated Name" Content="New content"
  ```
- **Create a new record:**
  ```bash
  rau tweets
  ```
- **Output the schema:**
  ```bash
  rau tweets --schema
  ```
- **Output available fields:**
  ```bash
  rau tweets --fields
  ```
- **Output recent records:**
  ```bash
  rau tweets --recent
  ```

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This project is licensed under the MIT License.
```
