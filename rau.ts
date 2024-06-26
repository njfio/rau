const BASEPATH = "/Users/n/.rau/";

const completionSpec: Fig.Spec = {
    name: "rau",
    description: "CLI for interacting with Airtable",
    args: [
        {
            name: "config",
            description: "The name of the configuration to use",
            isOptional: false,
            generators: {
                script: [`awk`, `-F=`, `/^[a-zA-Z0-9_-]+/`, `${BASEPATH}config.toml`],
                postProcess: (out) => {
                    return out
                        .split("\n")
                        .filter((line) => line.trim() && !line.startsWith("api_key"))
                        .map((line) => ({
                            name: line.split("=")[0].trim(),
                            description: "Configuration name",
                        }));
                },
            },
        },
        {
            name: "record_id",
            description: "The ID of the record to update or query",
            isOptional: true,
            generators: {
                script: (tokens) => {
                    const config = tokens[1];
                    if (config) {
                        return [`${BASEPATH}rau_fetch_records.sh`, config];
                    }
                    return [""];
                },
                postProcess: (out) => {
                    return out.split("\n").map((line) => {
                        const [id, name] = line.split(",", 2);
                        return {
                            name: name.replace(/"/g, "").trim(),
                            insertValue: id,
                            description: id,
                        };
                    });
                },
            },
        },
        {
            name: "fields",
            description: "Fields to update in key=value format or fields to query for their values",
            isOptional: true,
            isVariadic: true,
            generators: {
                script: (tokens) => {
                    const config = tokens[1];
                    if (config) {
                        return [`${BASEPATH}rau_fetch_fields.sh`, config];
                    }
                    return [""];
                },
                postProcess: (out) => {
                    return out.split("\n").map((line) => ({
                        name: line.trim(),
                        description: "Field name",
                    }));
                },
            },
        },
    ],
};

export default completionSpec;
const BASEPATH = "/Users/n/.rau/";

const completionSpec: Fig.Spec = {
    name: "rau",
    description: "CLI for interacting with Airtable",
    args: [
        {
            name: "config",
            description: "The name of the configuration to use",
            isOptional: false,
            generators: {
                script: [`awk`, `-F=`, `/^[a-zA-Z0-9_-]+/`, `${BASEPATH}config.toml`],
                postProcess: (out) => {
                    return out
                        .split("\n")
                        .filter((line) => line.trim() && !line.startsWith("api_key"))
                        .map((line) => ({
                            name: line.split("=")[0].trim(),
                            description: "Configuration name",
                        }));
                },
            },
        },
        {
            name: "record_id",
            description: "The ID of the record to update or query",
            isOptional: true,
            generators: {
                script: (tokens) => {
                    const config = tokens[1];
                    if (config) {
                        return [`${BASEPATH}rau_fetch_records.sh`, config];
                    }
                    return [""];
                },
                postProcess: (out) => {
                    return out.split("\n").map((line) => {
                        const [id, name] = line.split(",", 2);
                        return {
                            name: name.replace(/"/g, "").trim(),
                            insertValue: id,
                            description: id,
                        };
                    });
                },
            },
        },
        {
            name: "fields",
            description: "Fields to update in key=value format or fields to query for their values",
            isOptional: true,
            isVariadic: true,
            generators: {
                script: (tokens) => {
                    const config = tokens[1];
                    if (config) {
                        return [`${BASEPATH}rau_fetch_fields.sh`, config];
                    }
                    return [""];
                },
                postProcess: (out) => {
                    return out.split("\n").map((line) => ({
                        name: line.trim(),
                        description: "Field name",
                    }));
                },
            },
        },
    ],
};

export default completionSpec;
