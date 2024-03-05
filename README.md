# Windough

An easy-to-use command-line utility for saving and loading window arrangements on Windows

## Features

-   Save and load window arrangements
-   Customisable application launch arguments
-   Clear data format

## Installation

-   Download `windough.exe` from the [Releases tab on GitHub](https://github.com/SirGolem/windough/releases).
-   Move `windough.exe` to wherever you want to on your drive
-   [Optional] Add the path to the **directory** containing `windough.exe` to your system's `PATH` environment variable to allow it to be used from any terminal

## Usage

-   System-wide: `windough <command>`
-   From parent directory: `windough.exe <command>` or `./windough.exe <command>`

Run `windough help` for a list of commands and arguments

## Configuration

The Windough configuration file (`config.json`) can be found in the config directory opened by `windough open-dir --config`

### Values

-   **retry_count** (usize) - A positive integer that determines how many times repositioning and resizing of windows will be attempted
    -   Default: `5`
-   **retry_interval** (usize) - A positive integer that determines how long will be waited (in milliseconds) between each attempt to reposition and resize windows
    -   Default: `750`
