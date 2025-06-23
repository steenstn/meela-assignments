# Backend template

## Setup

1. Install sqlx cli

    ```sh
    $ cargo install sqlx-cli
    ```

2. Create the database.

    ```sh
    $ sqlx db create
    ```

3. Run sql migrations

    ```sh
    $ sqlx migrate run
    ```

## Usage

Start the server

```
cargo run
```
