# Self-Describing Telemetry Protocol (SDTP)

In order to be able to pass telemetry to the API without needing a defined configuration to decode the data, we need a self-describing configuration.

## Schema

```json
{
    "timestamp": 123456789,
    "dataset": "parent_name",
    "name": "channel_name",
    "value": "number or string",
    "indexes": {
        "index1": "index_value",
        "index2": "index_value"
    },
    "states": {
        0: "State1",
        1: "State2",
        2: "State3",
        3: "State4"
    },
    "description": "This is a sample description of the measurement"
}
```

### Timestamp
`timestamp` represents the time of the measurement in a Unix timestamp.

### Dataset
`dataset` is the name of the dataset that the channel comes from.

### Name
`name` is the name of the value.

### Value
`value` to be passed into the database. Can be a string or a number.

### Indexes
The `indexes` section is a dictionary to assign values to certain indexes for the specific measurement.

### [Optional] States
`states` is a dictionary that allows you to tag state names to certain values. If the `states` section exists but no index matches, it will be considered as `unknown`. It is optional.

### [Optional] Description
`description` allows you to provide a string description of the measurement. It is optional.