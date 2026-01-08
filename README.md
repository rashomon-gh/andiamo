# andiamo

A CLI tool for managing git repositories with dual remotes (origin and mirror).

## CLI options

- `--init`: Initialize a git repository in the current working directory
- `--add-remotes`: Add two remotes (origin and mirror) to the repository
- `--push`: Push changes to both origin and mirror remotes
- `--help`: To list the options above

## Installation

Build the project:

```bash
cargo build --release
```

The binary will be available at `./target/release/andiamo`.

## Usage

### Initialize a new repository

```bash
andiamo --init
```

This will:
- Check if a git repository already exists in the current directory
- If it exists, report that a repo already exists
- If it doesn't exist, initialize a new git repository

### Add remotes

```bash
andiamo --add-remotes
```

This will:
- Prompt you for the URL for the 'origin' remote
- Prompt you for the URL for the 'mirror' remote
- Check if remotes already exist
- Add only the remotes that don't already exist
- Report which remotes were added

### Push to both remotes

```bash
andiamo --push
```

This will:
- Push the current git changes to both origin and mirror remotes
- Report the status of each push operation

### Combine commands

You can combine multiple flags:

```bash
andiamo --init --add-remotes --push
```

## Example Workflow

1. Initialize a new repository:
   ```bash
   cd /path/to/your/project
   andiamo --init
   ```

2. Add your remotes:
   ```bash
   andiamo --add-remotes
   ```
   You'll be prompted to enter:
   - URL for the 'origin' remote 
   - URL for the 'mirror' remote

3. Make your changes and commit them using git:
   ```bash
   git add .
   git commit -m "Initial commit"
   ```

4. Push to both remotes:
   ```bash
   andiamo --push
   ```
