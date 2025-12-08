# Weather CLI 🌤️

![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Version](https://img.shields.io/badge/version-0.1.1-green.svg)

**Weather CLI** is a robust, extensible command-line application for fetching real-time and historical weather data. Built with Rust, it features a modular architecture that separates the CLI interface from the underlying weather providers, allowing for easy extension and integration.

## ✨ Features

- **Multi-Provider Support**: Switch seamlessly between different weather services (OpenWeather, WeatherAPI).
- **Extensible Architecture**: Core logic is decoupled into a workspace library (`weather-providers`).
- **Smart Aliasing**: Save frequently used locations with short names (e.g., `home` -> "London, UK").
- **Date Parsing**: Support for fetching weather for specific dates.
- **Persistent Configuration**: Automatically saves API keys, aliases, and preferences.
- **Mock Mode**: Built-in mock providers for offline testing and development.

## 🚀 Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Cargo

### Building from Source

Clone the repository and install the binary:

```bash
git clone https://github.com/AnatoliiShliakhto/weather.git
cd weather 
cargo install --path weather
```

Or build the release binary manually:

```bash
cargo build --release
```

The binary will be located at `target/release/weather`.

## ⚙️ Configuration

Before fetching real weather data, you need to configure at least one provider with an API key.

### Managing Providers

1.  **List available providers:**
    ```bash
    weather provider --list
    ```

2.  **Set an API Key:**
    ```bash
    # For WeatherAPI (wa)
    weather provider WeatherAPI --key <YOUR_API_KEY>
    
    # For OpenWeather (ow)
    weather provider ow -k <YOUR_API_KEY>
    ```

3.  **Set a Default Provider:**
    When you set a key, that provider automatically becomes the default. You can change it manually:
    ```bash
    # Switch default to WeatherAPI
    weather provider wa
    ```

## 📖 Usage

### Fetching Weather

**Basic usage:**

```bash
weather get "London, UK"
```

**Using a specific provider for one request:**

```bash
weather get "Paris" --provider grpc
```

**Fetching weather for a specific date:**

```bash
weather get "New York" --date 2023-12-25
```

### Managing Aliases

Save frequently typed addresses to save time.

**Set an alias:**

```bash
weather alias home --address "UK, London" 
weather alias work -a "Paris"
```

**Use an alias:**

```bash
weather get home
```
* *If you run `weather get` without arguments, it uses the default alias.*

**Sets `home` as the default alias:**

```bash
weather alias home
```

**List and remove aliases:**

```bash
weather alias --list
weather alias --remove work
```

### Debugging

Enable verbose logging to inspect internal state and API requests:

```bash
weather get "Berlin" --debug
```

## 🏗️ Architecture

The project is organized as a Cargo Workspace with a clean separation of concerns:

- **`weather-cli`**: The binary crate handling command-line arguments (using `clap`), configuration management, and user interaction.
- **`weather-providers`**: A library crate defining the `WeatherProvider` trait. It implements the logic for specific APIs (OpenWeather, WeatherAPI, Mock) and handles data normalization.

### Project Structure
```text
weather/ 
├── weather-cli/ # CLI Application 
│ ├── src/handlers/ # Command logic (get, provider, alias) 
│ ├── src/models/ # Argument parsing structures 
│ └── src/common/ # Config, State, and Logging 
└── weather-providers/ # Core Logic Library 
  ├── src/providers/ # API implementations 
  └── src/models/ # Data transfer objects

```

## 🧪 Development

### Running Tests

The project includes unit tests and integration tests.

**Run all tests**

```bash 
cargo test
```

**Run specific integration tests for the CLI**

```bash
cargo test --test cli_tests
```


### Local Development Files

When running in debug mode, configuration files and logs are stored in the `.dev` directory in the project root to avoid polluting your system configuration.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1.  Fork the project
2.  Create your feature branch (`git checkout -b feature/AmazingFeature`)
3.  Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4.  Push to the branch (`git push origin feature/AmazingFeature`)
5.  Open a Pull Request

## 📄 License

This project is licensed under the MIT License.

---

*Authors: Anatolii Shliakhto*