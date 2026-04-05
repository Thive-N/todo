# Todo

## Overview

a simple command-line interface (CLI) tool for managing a to-do list. It allows you to add tasks, list them, mark tasks as done, and remove tasks. The application uses Rust as the backend and stores tasks in a JSON file located at `~/.config/todolist.json`.

## Installation

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/Thive-N/todo.git
   cd todo
   ```

2. **Build the Application:**
   ```bash
   cargo build
   cargo install --path .
   ```

3. **Run the Application:**
   ```bash
   todo
   ```

## Usage

### Add a Task
To add a task, you can use the `add` subcommand. You can optionally specify a priority and due date.

```bash
todo add "Buy groceries" --priority high --due tomorrow
```

### List Tasks
To list all tasks, use the `list` subcommand.

```bash
todo list
```

### Mark a Task as Done
To mark a task as done, use the `done` subcommand with the index of the task.

```bash
todo done 0
```

### Remove a Task
To remove a task, use the `remove` subcommand with the index of the task.

```bash
todo remove 1
```

## Command Reference

### `add`
Add a new task to the list.

- **Options:**
  - `--priority <PRIORITY>`: Set the priority (high, medium, low). Default is low.
  - `--due <DUE_DATE>`: Set the due date for the task.

- **Example:**
  ```bash
  todo add "Buy groceries" --priority high --due tomorrow
  ```

### `list`
List all tasks in the to-do list.

- **Example:**
  ```bash
  todo list
  ```

### `done`
Mark a task as done.

- **Options:**
  - `<INDEX>`: Index of the task to mark as done.

- **Example:**
  ```bash
  todo done 0
  ```

### `remove`
Remove a task from the list.

- **Options:**
  - `<INDEX>`: Index of the task to remove.

- **Example:**
  ```bash
  todo remove 1
  ```