# Contributing to TapTap PC SDK

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing.

## Development Setup

### Prerequisites

- **Rust** (stable toolchain)
- **Node.js** >= 20
- **pnpm** >= 9
- **LLVM/Clang** (for bindgen)
- **Windows** x64 (required for building the native module)

### Getting Started

1. Clone the repository:
   ```bash
   git clone https://github.com/dsh0416/tapsdk-pc.js.git
   cd tapsdk-pc-js
   ```

2. Install dependencies:
   ```bash
   pnpm install
   ```

3. Build the Rust crates:
   ```bash
   cargo build --workspace
   ```

4. Build the Node.js module:
   ```bash
   cd packages/tapsdk-pc-js
   pnpm run build
   ```

5. Run the documentation site locally:
   ```bash
   pnpm docs:dev
   ```

## Project Structure

```
tapsdk-pc-js/
├── .github/              # GitHub Actions workflows
├── docs/                 # VitePress documentation
├── reference/            # Original SDK files
├── crates/
│   ├── tapsdk-pc-sys/   # Raw FFI bindings
│   └── tapsdk-pc/       # Safe Rust wrapper
└── packages/
    └── tapsdk-pc-js/    # Node.js bindings
```

## Development Workflow

### Making Changes

1. Create a new branch from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes following the coding standards below.

3. Test your changes:
   ```bash
   # Rust tests
   cargo test --workspace

   # Node.js type checking
   cd packages/tapsdk-pc-js
   pnpm run typecheck
   ```

4. Commit your changes with a descriptive message:
   ```bash
   git commit -m "feat: add new feature"
   ```

5. Push and create a pull request.

### Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

### Coding Standards

#### Rust

- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Add documentation comments for public APIs

#### TypeScript

- Follow the existing code style
- Add JSDoc comments for public APIs
- Ensure type checking passes

## Testing

### Rust Tests

```bash
cargo test --workspace
```

Note: Some tests require the TapTap client to be running.

### Integration Tests

Integration tests are located in `crates/tapsdk-pc/tests/`. These tests verify the SDK works correctly with the native DLL.

## Documentation

Documentation is built with VitePress and located in the `docs/` folder.

### Running Locally

```bash
pnpm docs:dev
```

### Building

```bash
pnpm docs:build
```

### Adding New Pages

1. Create a new `.md` file in the appropriate directory
2. Update `docs/.vitepress/config.ts` to add it to the sidebar

## Questions?

If you have questions, please open an issue on GitHub.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
