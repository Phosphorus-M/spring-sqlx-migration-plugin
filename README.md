# Spring Sqlx Migration Plugin

It's just a plugin to execute the migrations of Sqlx into SpringRS

Just add into the app.toml the initial config for this plugin:

```toml
[sqlx]
uri = "postgres://postgres:xudjf23adj213@127.0.0.1:5432"

[sqlx-migration]
migration_folder = "./migrations"
``` 

And into your `main.rs`

```rust
#[auto_config(WebConfigurator)] // auto config web router
#[tokio::main]
async fn main() {
    App::new()
        .add_plugin(SqlxPlugin) // Add plug-in
        .add_plugin(SqlxMigrationPlugin) // Add plug-in
        .add_plugin(WebPlugin)
        .run().await
}
```