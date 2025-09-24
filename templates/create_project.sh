#!/bin/bash

# World Simulator Project Creation Script
# This script creates a new world simulator project from templates

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
TEMPLATE="basic_sim"
PROJECT_NAME=""
TARGET_DIR=""

# Available templates
declare -A TEMPLATES=(
    ["basic_sim"]="Basic world simulation with core features"
    ["ai_extension"]="Template for adding new AI behaviors"
    ["economic_mod"]="Template for economic simulation mods"
    ["world_gen"]="Template for custom world generation"
    ["perf_test"]="Template for performance testing"
    ["web_integration"]="Template for web-based simulators"
)

# Print usage
print_usage() {
    echo -e "${BLUE}World Simulator Project Creator${NC}"
    echo ""
    echo "Usage: $0 [OPTIONS] PROJECT_NAME"
    echo ""
    echo "Options:"
    echo "  -t, --template TEMPLATE    Project template (default: basic_sim)"
    echo "  -d, --dir DIRECTORY        Target directory (default: PROJECT_NAME)"
    echo "  -h, --help                 Show this help message"
    echo ""
    echo "Available templates:"
    for template in "${!TEMPLATES[@]}"; do
        echo "  ${YELLOW}$template${NC}: ${TEMPLATES[$template]}"
    done
    echo ""
    echo "Example:"
    echo "  $0 my_simulation --template basic_sim"
    echo "  $0 ai_experiment --template ai_extension"
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -t|--template)
                TEMPLATE="$2"
                shift 2
                ;;
            -d|--dir)
                TARGET_DIR="$2"
                shift 2
                ;;
            -h|--help)
                print_usage
                exit 0
                ;;
            -*)
                echo -e "${RED}Error: Unknown option $1${NC}"
                print_usage
                exit 1
                ;;
            *)
                if [[ -z "$PROJECT_NAME" ]]; then
                    PROJECT_NAME="$1"
                else
                    echo -e "${RED}Error: Multiple project names specified${NC}"
                    print_usage
                    exit 1
                fi
                shift
                ;;
        esac
    done

    # Validate project name
    if [[ -z "$PROJECT_NAME" ]]; then
        echo -e "${RED}Error: Project name is required${NC}"
        print_usage
        exit 1
    fi

    # Validate template
    if [[ -z "${TEMPLATES[$TEMPLATE]}" ]]; then
        echo -e "${RED}Error: Unknown template '$TEMPLATE'${NC}"
        echo "Available templates:"
        for template in "${!TEMPLATES[@]}"; do
            echo "  $template"
        done
        exit 1
    fi

    # Set target directory
    if [[ -z "$TARGET_DIR" ]]; then
        TARGET_DIR="$PROJECT_NAME"
    fi
}

# Check if template directory exists
check_template() {
    local template_dir="$(dirname "$0")/$TEMPLATE"
    if [[ ! -d "$template_dir" ]]; then
        echo -e "${RED}Error: Template directory '$template_dir' not found${NC}"
        exit 1
    fi
}

# Create target directory
create_directory() {
    if [[ -d "$TARGET_DIR" ]]; then
        echo -e "${YELLOW}Warning: Directory '$TARGET_DIR' already exists${NC}"
        read -p "Do you want to overwrite? [y/N] " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "Cancelled"
            exit 0
        fi
        rm -rf "$TARGET_DIR"
    fi

    mkdir -p "$TARGET_DIR"
    echo -e "${GREEN}Created directory: $TARGET_DIR${NC}"
}

# Copy template files
copy_template_files() {
    local template_dir="$(dirname "$0")/$TEMPLATE"

    echo -e "${BLUE}Copying template files...${NC}"

    # Copy all files from template
    cp -r "$template_dir"/* "$TARGET_DIR/"

    # Make scripts executable
    if [[ -f "$TARGET_DIR/build.sh" ]]; then
        chmod +x "$TARGET_DIR/build.sh"
    fi
    if [[ -f "$TARGET_DIR/run.sh" ]]; then
        chmod +x "$TARGET_DIR/run.sh"
    fi
}

# Customize project files
customize_files() {
    echo -e "${BLUE}Customizing project files...${NC}"

    # Update Cargo.toml
    if [[ -f "$TARGET_DIR/Cargo.toml" ]]; then
        sed -i.tmp "s/basic_simulation/$PROJECT_NAME/g" "$TARGET_DIR/Cargo.toml"
        sed -i.tmp "s/Basic Simulation Template/$PROJECT_NAME Simulation/g" "$TARGET_DIR/Cargo.toml"
        rm -f "$TARGET_DIR/Cargo.toml.tmp"
    fi

    # Update README.md
    if [[ -f "$TARGET_DIR/README.md" ]]; then
        sed -i.tmp "s/Basic World Simulation Template/$PROJECT_NAME Simulation/g" "$TARGET_DIR/README.md"
        sed -i.tmp "s/basic_sim/$PROJECT_NAME/g" "$TARGET_DIR/README.md"
        rm -f "$TARGET_DIR/README.md.tmp"
    fi

    # Update main.rs if it exists
    if [[ -f "$TARGET_DIR/src/main.rs" ]]; then
        sed -i.tmp "s/Starting Basic World Simulation/Starting $PROJECT_NAME Simulation/g" "$TARGET_DIR/src/main.rs"
        rm -f "$TARGET_DIR/src/main.rs.tmp"
    fi
}

# Initialize Git repository
init_git() {
    echo -e "${BLUE}Initializing Git repository...${NC}"
    cd "$TARGET_DIR"
    git init

    # Create initial commit
    git add .
    git commit -m "Initial commit: Create $PROJECT_NAME from $TEMPLATE template"

    echo -e "${GREEN}Git repository initialized${NC}"
}

# Install dependencies
install_deps() {
    echo -e "${BLUE}Installing dependencies...${NC}"
    cd "$TARGET_DIR"

    # Check if Cargo is available
    if ! command -v cargo &> /dev/null; then
        echo -e "${YELLOW}Warning: Cargo not found. Please install Rust and Cargo manually.${NC}"
        return
    fi

    # Build the project
    if cargo build; then
        echo -e "${GREEN}Dependencies installed successfully${NC}"
    else
        echo -e "${RED}Error: Failed to build project${NC}"
        exit 1
    fi
}

# Create build script
create_build_script() {
    cat > "$TARGET_DIR/build.sh" << 'EOF'
#!/bin/bash

# Build script for world simulator project

set -e

echo "Building project..."

# Build in release mode
cargo build --release

echo "Build complete!"
echo "Binary available at: target/release/$(grep '^name = ' Cargo.toml | cut -d'"' -f2)"
EOF

    chmod +x "$TARGET_DIR/build.sh"
}

# Create run script
create_run_script() {
    cat > "$TARGET_DIR/run.sh" << 'EOF'
#!/bin/bash

# Run script for world simulator project

set -e

# Parse command line arguments
BUILD=false
CONFIG=""
DEBUG=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -b|--build)
            BUILD=true
            shift
            ;;
        -c|--config)
            CONFIG="$2"
            shift 2
            ;;
        -d|--debug)
            DEBUG=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  -b, --build      Build before running"
            echo "  -c, --config FILE Configuration file"
            echo "  -d, --debug      Enable debug mode"
            echo "  -h, --help       Show this help"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Build if requested
if [[ "$BUILD" == true ]]; then
    echo "Building project..."
    cargo build --release
fi

# Set up run command
RUN_CMD="cargo run"

if [[ "$DEBUG" == true ]]; then
    RUN_CMD="$RUN_CMD --features debug"
fi

if [[ -n "$CONFIG" ]]; then
    RUN_CMD="$RUN_CMD -- --config $CONFIG"
fi

# Run the simulation
echo "Starting simulation..."
exec $RUN_CMD
EOF

    chmod +x "$TARGET_DIR/run.sh"
}

# Create development workflow
create_dev_workflow() {
    cat > "$TARGET_DIR/DEVELOPMENT.md" << EOF
# Development Workflow

This document describes the development workflow for $PROJECT_NAME.

## Building

\`\`\`bash
# Development build
cargo build

# Release build
cargo build --release

# Using build script
./build.sh
\`\`\`

## Running

\`\`\`bash
# Run with default settings
./run.sh

# Run with custom config
./run.sh --config config.toml

# Run in debug mode
./run.sh --debug

# Build and run
./run.sh --build
\`\`\`

## Testing

\`\`\`bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run benchmarks
cargo bench
\`\`\`

## Development Tools

### Code Formatting
\`\`\`bash
cargo fmt
\`\`\`

### Linting
\`\`\`bash
cargo clippy
\`\`\`

### Documentation
\`\`\`bash
# Generate documentation
cargo doc --no-deps

# Open documentation in browser
cargo doc --no-deps --open
\`\`\`

## Project Structure

\`\`\`
$PROJECT_NAME/
├── src/
│   ├── main.rs          # Main entry point
│   ├── config.rs        # Configuration management
│   ├── systems.rs       # Simulation systems
│   ├── components.rs    # ECS components
│   └── lib.rs           # Library exports
├── config.toml          # Configuration file
├── Cargo.toml           # Project dependencies
├── build.sh            # Build script
├── run.sh              # Run script
└── DEVELOPMENT.md       # This file
\`\`\`

## Next Steps

1. **Customize the configuration** in \`config.toml\`
2. **Add new components** in \`src/components.rs\`
3. **Implement new systems** in \`src/systems.rs\`
4. **Extend the AI** with new behaviors
5. **Add tests** for new functionality
6. **Update documentation** as needed

Happy coding! 🚀
EOF
}

# Print success message
print_success() {
    echo -e "${GREEN}Project created successfully!${NC}"
    echo ""
    echo "Project location: $TARGET_DIR"
    echo "Template: $TEMPLATE"
    echo ""
    echo "Next steps:"
    echo "  cd $TARGET_DIR"
    echo "  ./run.sh        # Run the simulation"
    echo "  ./build.sh      # Build the project"
    echo "  cargo test      # Run tests"
    echo ""
    echo "Happy simulating! 🎮"
}

# Main execution
main() {
    parse_args "$@"
    check_template
    create_directory
    copy_template_files
    customize_files
    init_git
    install_deps
    create_build_script
    create_run_script
    create_dev_workflow
    print_success
}

# Run main function
main "$@"