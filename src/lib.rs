use serde::Deserialize;
use spring::async_trait;
use spring::config::{ConfigRegistry, Configurable};
use spring::plugin::ComponentRegistry;
use spring::tracing::{error, info};
use spring::{app::AppBuilder, plugin::Plugin};
use spring_sqlx::{ConnectPool, SqlxPlugin};
use sqlx::migrate::Migrator;

pub struct SqlxMigrationPlugin;

#[async_trait]
impl Plugin for SqlxMigrationPlugin {
    async fn build(&self, app: &mut AppBuilder) {
        let Ok(SqlxMigrationConfig { migration_folder }) = app.get_config::<SqlxMigrationConfig>() else {
            error!("SqlxMigrationPlugin require the migration_folder config");
            return;
        };

        let pool = app.get_component::<ConnectPool>().expect("sqlx connect pool not exists");

        let Ok(migration_path) = std::path::absolute(migration_folder) else {
            error!("Folder not found");
            return;
        };

        let Ok(migrator) = Migrator::new(migration_path).await else {
            error!("SQLX Migration plugin load failed");
            return;
        };

        let count = migrator.iter().count();
        info!("Migrations to run: {count}");

        migrator.iter().for_each(|migration| {
            let description = migration.description.clone();
            let version = migration.version;
            info!("{description} {version}");
        });

        if let Err(error) = migrator.run(&pool).await {
            error!("Something goes wrong executing the migrations!");
            error!("{error:#}")
        }

    }

    fn dependencies(&self) -> Vec<&str> {
        vec![std::any::type_name::<SqlxPlugin>()]
    }
}

#[derive(Debug, Configurable, Deserialize)]
#[config_prefix = "sqlx-migration"]
pub struct SqlxMigrationConfig {
    migration_folder: String,
}
