# Crazy Train

**Crazy Train**  designed to perform randomized and fuzz execution of the [Loco CLI](https://loco.rs) to discover unforeseen sequences of steps and parameters that users run, which may lead to unexpected errors. This library aims to facilitate reproducible test plan runs, ensuring your CLI behaves as expected under various scenarios.

## Features

- **Randomized Execution**: Execute the Loco CLI with random parameters and sequences to explore unexpected behaviors.
- **Fuzz Testing**: Identify edge cases and potential bugs by fuzzing input to the CLI.
- **Reproducible Tests**: Create a test plan that can be repeated to ensure consistency in test results.
- **Error Discovery**: Capture and report unforeseen errors encountered during execution.

## Installation

To add **Crazy Train** to your project, include it in your `Cargo.toml`:

```toml
[dependencies]
crazy-train = "0.1.0"  # Replace with the latest version
```

## Usage
[Here’s](./examples/run.rs) a quick example of how to use Crazy Train in your project


## Contributing
Contributions are welcome! If you have suggestions or find bugs, please open an issue or submit a pull request. Make sure to follow the contribution guidelines.