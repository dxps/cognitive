## Operations :: Database

This section includes database related scripts and files to migrate (apply changes) the database.

<br/>

### Prerequisites

1. PostgreSQL CLI Client

    - On macOS, it can be installed with Homebrew using:<br/>
        ```shell
        brew install libpq
        brew link --force libpq
        ```

1. SQLX CLI
    - Install it using `cargo install --version=0.8.6 sqlx-cli --no-default-features --features native-tls,postgres`
