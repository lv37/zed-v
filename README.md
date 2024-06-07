# Zed V

This extension adds support for the [V programming language](https://vlang.org/) to the [Zed editor](https://zed.dev).

## Runnables
By default `runnables` won't do anything as they only run appropriately tagged tasks.
To use runnables, you must add one of these supported tags to your tasks:
 - `v-main`: Runs on the `main` function in a V file.
 - `v-test`: Runs on functions whose names start with `test_`

## Example
Some example tasks to get you started. Place these in your `tasks.json` file.
```json
[
  {
    "label": "V run main",
    "command": "v",
    "args": ["run", "$ZED_FILE"],
    "tags": ["v-main"],
    "use_new_terminal": false,
    "reveal": "always"
  },
  {
    "label": "V test",
    "command": "v",
    "args": ["test", "$ZED_DIRNAME"],
    "tags": ["v-test"],
    "use_new_terminal": false,
    "reveal": "always",
  }
]
```