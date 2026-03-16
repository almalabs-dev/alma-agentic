# Contributing to alma-agentic

Thank you for your interest in contributing to alma-agentic.

## Getting Started

```bash
git clone https://github.com/almalabs-dev/alma-agentic
cd alma-agentic
cargo test
```

## Code Quality

We enforce `clippy` and `fmt` on all contributions:

```bash
cargo clippy --all-features --all-targets
cargo fmt -- --check
```

## Pull Requests

- Open an issue first to discuss the change
- Keep PRs small and focused
- Use [Conventional Commits](https://conventionalcommits.org/en/v1.0.0) format
- Include tests or examples for new functionality
- Use docstrings on all public items

## Code Guidelines

- Use full syntax for trait bounds where possible
- New public items must include documentation
- PRs adding functionality must include relevant tests

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
