# GitHub Actions Workflow Documentation

This document provides a detailed explanation of the GitHub Actions workflow defined in the `test.yaml` file. The workflow is designed to automate the testing process for the AnasOS kernel project. It triggers on pushes and pull requests to the `main` branch, installs necessary dependencies, and runs the tests to ensure the integrity of the codebase.

## Workflow Overview

The workflow is named "Tests" and is triggered by two events:
- A push to the `main` branch.
- A pull request targeting the `main` branch.

When triggered, the workflow runs a series of steps on an `ubuntu-latest` runner to set up the environment, install dependencies, and execute the tests.

## Detailed Breakdown

### `name: Tests`

This specifies the name of the workflow.

### `on`

Defines the events that trigger the workflow:
- `push`: Triggers the workflow when there is a push to the `main` branch.
- `pull_request`: Triggers the workflow when a pull request is opened, synchronized, or reopened targeting the `main` branch.

### `jobs`

Defines the jobs that will be run as part of the workflow. In this case, there is a single job named `build`.

#### `build`

This job runs on the `ubuntu-latest` virtual environment.

##### `steps`

The steps define the sequence of actions to be performed in the `build` job.

1. **Checkout the repository**
```yaml
- uses: actions/checkout@v4
```
This step uses the `actions/checkout` action to clone the repository to the runner.

2. **Install additional dependencies**
```yaml
- name: Install additional dependencies
    run: |
    sudo apt update
    sudo apt install -y nasm grub-pc-bin grub-common make mtools xorriso
    rustup update nightly
    rustup target add x86_64-unknown-none --toolchain nightly
    rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
```
This step installs the necessary dependencies for building and testing the AnasOS kernel. It updates the package list, installs various tools such as `nasm`, `grub-pc-bin`, `grub-common`, `make`, `mtools`, and `xorriso`. It also updates Rust to the nightly version, adds the `x86_64-unknown-none` target, and installs the Rust source component.

3. **Run Tests**
```yaml
- name: Run Tests
    run: make test
```
This step runs the tests using the `make test` command to ensure that the codebase is functioning correctly.

## Conclusion

The GitHub Actions workflow described in this document provides an automated and efficient way to test the AnasOS kernel project. By triggering on pushes and pull requests to the `main` branch, it ensures that any changes to the codebase are thoroughly tested. The workflow sets up the necessary environment, installs dependencies, and runs tests, helping to maintain the integrity and stability of the project. This automation not only saves time but also enhances the reliability of the development process.