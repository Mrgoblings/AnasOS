# Continuous Integration (CI) Automation

Continuous Integration (CI) automation is essential for maintaining a stable and reliable codebase. By automating the testing and release processes, we ensure that every change is thoroughly tested before being merged, and that new releases are created seamlessly when pull requests are merged. This helps in catching bugs early, maintaining code quality, and speeding up the development process.

## Release Workflow

The release workflow is triggered when a pull request is merged into the main branch. It automatically determines the new version based on the pull request title, creates a new release, and uploads the build artifacts. This ensures that every change is properly versioned and released without manual intervention.

[Read more about the release workflow](release.md)

## Test Workflow

The test workflow runs on every push and pull request to the main branch. It installs necessary dependencies and runs the project's tests to ensure that new changes do not break existing functionality. This is crucial for maintaining a stable codebase and preventing regressions.

[Read more about the test workflow](test.md)