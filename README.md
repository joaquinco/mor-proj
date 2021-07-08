# Metaherustics for Network Optimization - Project

## Problem description

The problem we are trying to solve here is the Vehicle routing problem with a heterogeneous fleet and time windows. It's a vehicle routing problem where vehicles need to visit clients to satisfy their demand withing a fixed time window. All vehicles depart from and arrive to the same node (the deposit).

Optimization can be made over cost: sum of fixed cost plus variable cost * distance for each vehicle, or over distance: sum of the route distance for each vehicle. This behavior is specified by the configuration entry `optimize_cost: {true|false}`, the default is true.

## Software requirements

- rust: 1.4.X,1.5.X

## Debug

To compile and run the program just run:

```bash
cargo run
```

## Release build

### Compilation

```bash
cargo build -j4 --release
```

After building, the self contained binary will be placed under `target/release`.

## Running

You'll require to pass at least a json file with the problem description.

A sample instance file is:

```json
{
  "name": "Simple 1",
  "source": 0,
  "penalty": 0.1,
  "deviation": 0.3,
  "distances": [
    [0, 10, 20, 30],
    [10, 0, 31, 14],
    [20, 31, 0, 7],
    [30, 14, 7, 0]
  ],
  "vehicle_definitions": [
    { "count": 10, "capacity": 100, "fixed_cost": 30, "variable_cost": 1 }
  ],
  "clients": [
    { "demand": 0, "earliest": 0, "latest": 10000, "service_time": 0 },
    { "demand": 10, "earliest": 50, "latest": 110, "service_time": 50 },
    { "demand": 20, "earliest": 100, "latest": 200, "service_time": 100 },
    { "demand": 15, "earliest": 100, "latest": 150, "service_time": 330 }
  ]
}
```

And a sample configuration file:

```json
{
  "iters": 300,
  "number_of_threads": 4,
  "grasp_config": {
    "local_search_iters": 10,
    "weight_configs": [
      {
        "time_weight": 0.2,
        "distance_weight": 0.3,
        "wait_time_weight": 0.5,
        "config_weight": 0.9,
        "display_name": "Highly likely"
      },
      {
        "time_weight": 0.2,
        "distance_weight": 0.5,
        "wait_time_weight": 0.3,
        "config_weight": 0.1,
        "display_name": "Unlikely"
      }
    ]
  }
}
```
