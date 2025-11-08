# 🍹 Smoothie - MCTS UI Adaptation

A Monte Carlo Tree Search (MCTS) implementation for UI adaptation with a modern web interface.

## Features

- **Web-based UI**: Clean, modern interface built with HTML/CSS/JavaScript
- **Rust Backend**: High-performance backend using Actix-web framework
- **Responsive Design**: Works seamlessly on desktop, tablet, and mobile devices
- **Interactive Controls**: Configure and run MCTS simulations with custom parameters
- **Real-time Results**: View simulation results instantly in the browser

## Getting Started

### Prerequisites

- Rust (2021 edition or later)
- Cargo package manager

### Installation

1. Clone the repository:
```bash
git clone https://github.com/agxmbhir/smoothie.git
cd smoothie
```

2. Build the project:
```bash
cargo build --release
```

### Running the Application

1. Start the web server:
```bash
cd adaptation
cargo run
```

2. Open your browser and navigate to:
```
http://127.0.0.1:8080
```

## Usage

1. **Configure Simulation Parameters**:
   - Set the number of iterations (1-10000)
   - Set the move limit (1-100)

2. **Run Simulation**:
   - Click the "Run Simulation" button to start
   - View results in the "Simulation Results" section

3. **Reset**:
   - Click "Reset" to clear results and restore default values

## Project Structure

```
smoothie/
├── adaptation/
│   ├── src/
│   │   ├── main.rs       # Web server and API endpoints
│   │   ├── node.rs       # MCTS node implementation
│   │   ├── mcts.rs       # MCTS algorithm (WIP)
│   │   └── mod.rs        # Module declarations
│   ├── static/
│   │   ├── index.html    # Main UI page
│   │   ├── styles.css    # Responsive styling
│   │   └── script.js     # Frontend logic
│   └── Cargo.toml        # Package configuration
└── Cargo.toml            # Workspace configuration
```

## Technology Stack

- **Backend**: Rust + Actix-web
- **Frontend**: HTML5, CSS3, JavaScript (Vanilla)
- **Architecture**: RESTful API

## API Endpoints

### POST /api/run
Run an MCTS simulation with specified parameters.

**Request Body**:
```json
{
  "iterations": 100,
  "move_limit": 10
}
```

**Response**:
```json
{
  "status": "success",
  "message": "Simulation completed with 100 iterations and move limit of 10",
  "iterations": 100,
  "move_limit": 10
}
```

## Development

### Building
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

### Running in Development Mode
```bash
cd adaptation
cargo run
```

## About MCTS

Monte Carlo Tree Search (MCTS) is a search algorithm used in decision-making processes. The algorithm works through four main phases:

1. **Selection**: Traverse the tree to find the most promising node
2. **Expansion**: Add new child nodes to explore
3. **Simulation**: Run random playouts from the new node
4. **Backpropagation**: Update values back up the tree

## License

This project is part of the smoothie repository.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
