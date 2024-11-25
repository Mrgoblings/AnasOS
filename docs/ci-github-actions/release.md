# Release Workflow Documentation

This document provides a comprehensive overview of the GitHub Actions workflow defined in `release.yaml`. The workflow is designed to automate the release process of the AnasOS project. It triggers on pull requests merged into the `main` branch, determines the appropriate version increment based on the pull request title, creates a new release, and builds the project packages.

## Workflow Overview

The release workflow is triggered when a pull request is closed and merged into the `main` branch. It performs the following key steps:

1. Checks out the repository.
2. Fetches the tags to determine the highest release version.
3. Determines the version increment based on the pull request title.
4. Creates a new release with the incremented version.
5. Builds the project packages.
6. Uploads the build artifacts to the release.

## Detailed Breakdown

### Triggering the Workflow

```yaml
on:
  pull_request:
    branches: ["main"]
    types: [closed]
```

The workflow is triggered when a pull request targeting the `main` branch is closed.

### Permissions

```yaml
permissions:
  contents: write
  pull-requests: read
```

The workflow requires write access to the repository contents and read access to pull requests.

### Create Release Job

#### Job Conditions

```yaml
if: github.event.pull_request.merged == true && github.event.pull_request.base.ref == 'main'
```

The `create-release` job runs only if the pull request is merged into the `main` branch.

#### Steps

1. **Checkout Repository**

```yaml
- name: Checkout with GitHub
    uses: actions/checkout@v3
```

Checks out the repository to the runner.

2. **Fetch Tags**

```yaml
- name: Fetch tags
    run: git fetch --prune --unshallow --tags
```

Fetches all tags to determine the highest release version.

3. **Get Highest Release Version**

```yaml
- name: Get highest release version
    id: get_version
    run: |
        version=$(git tag --list 'v*' | sort -V | tail -n 1 || echo "v0.0.0")
        echo "TAG=$version" >> $GITHUB_ENV
        echo "Highest version is $version"
```

Determines the highest release version tag.

4. **Determine Version Increment**

```yaml
- name: Determine version increment
    id: determine_increment
    run: |
        major=$(echo "$TAG" | cut -d '.' -f1 | sed 's/v//')
        minor=$(echo "$TAG" | cut -d '.' -f2)
        patch=$(echo "$TAG" | cut -d '.' -f3)

        pr_title="${{ github.event.pull_request.title }}"
        if [[ "$pr_title" == *"[Major]"* ]]; then
                major=$((major + 1))
                minor=0
                patch=0
        elif [[ "$pr_title" == *"[Minor]"* ]]; then
                minor=$((minor + 1))
                patch=0
        elif [[ "$pr_title" == *"[Patch]"* ]]; then
                patch=$((patch + 1))
        else
                minor=$((minor + 1))
                patch=0
        fi

        TAG="v$major.$minor.$patch"
        echo "TAG=$TAG" >> $GITHUB_ENV
```

Determines the new version based on the pull request title. The version increment follows these rules:

- If the title contains `[Major]`, the major version is incremented, and the minor and patch versions are reset to 0.
- If the title contains `[Minor]`, the minor version is incremented, and the patch version is reset to 0.
- If the title contains `[Patch]`, the patch version is incremented.
- If none of these keywords are present, the minor version is incremented, and the patch version is reset to 0.

5. **Create Release**

```yaml
- name: Create Release
    id: release-action
    uses: ncipollo/release-action@v1
    with:
        tag: ${{ env.TAG }}
        name: Release ${{ env.TAG }}
        commit: ${{ github.sha }}
        body: ${{ github.event.pull_request.body }}
```

Creates a new release with the determined version.

6. **Output Release URL File**

```yaml
- name: Output Release URL File
    run: |
        echo "${{ steps.release-action.outputs.upload_url }}" > release_url.txt
```

Writes the release URL to a file for future use.

7. **Save Release URL File for Publish**

```yaml
- name: Save Release URL File for publish
    uses: actions/upload-artifact@v4
    with:
        name: release_url
        path: release_url.txt
```

Saves the release URL file as an artifact.

### Build Job

#### Job Dependencies

```yaml
needs: create-release
```

The `build` job depends on the successful completion of the `create-release` job.

#### Steps

1. **Checkout Repository**

```yaml
- name: Checkout with GitHub
    uses: actions/checkout@v4
```

Checks out the repository to the runner.

2. **Install Dependencies**

```yaml
- name: Install additional dependencies
    run: |
        sudo apt update
        sudo apt install -y nasm grub-pc-bin grub-common make mtools xorriso
        rustup update nightly
        rustup target add x86_64-unknown-none --toolchain nightly
        rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
```

Installs necessary dependencies for building the project.

3. **Build Project**

```yaml
- name: Build project
    run: make no-run
```

Builds the project using the `make` command with the `no-run` argument, which generates the image file to be included in the release.

4. **Archive Build Output**

```yaml
- name: Archive build output
    run: |
        zip -r AnasOS.iso.zip AnasOS.iso
```

This step archives the build output into a zip file. Although the build output is a single ISO file, archiving it has several benefits:

- **Compression**: Reduces the file size, which can save storage space and reduce upload/download times.
- **Compatibility**: Some systems and tools handle zip files more gracefully than raw ISO files.
- **Convenience**: Packaging the file in a zip format can make it easier for users to manage and extract the contents if needed.

5. **Load Release URL File**

```yaml
- name: Load Release URL File from release job
    id: download_release_info
    uses: actions/download-artifact@v4
    with:
        name: release_url
```

Downloads the release URL file from the `create-release` job.

6. **Get Release File Name & Upload URL**

```yaml
- name: Get Release File Name & Upload URL
    id: get_release_info
    shell: bash
    run: |
        value=`cat "${{steps.download_release_info.outputs.download-path}}/release_url.txt"`
        echo ::set-output name=upload_url::$value
```

Extracts the release upload URL from the file.

7. **Upload Release Asset**

```yaml
- name: Upload Release Asset
    uses: actions/upload-release-asset@v1
    env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
    with:
        upload_url: ${{ steps.get_release_info.outputs.upload_url }}
        asset_path: AnasOS.iso.zip
        asset_name: AnasOS.iso.zip
        asset_content_type: application/zip
```

Uploads the build artifact to the release.

## Conclusion

The release workflow for the AnasOS project automates the process of creating a new release and building the project packages. By leveraging GitHub Actions, the workflow ensures that every pull request merged into the `main` branch triggers a series of steps to determine the new version, create a release, and upload the build artifacts. This automation not only saves time but also reduces the potential for human error, ensuring a consistent and reliable release process.
