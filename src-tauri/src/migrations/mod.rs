// mod pokemon_migrations;

use std::{
    collections::HashMap,
    fs::{self, File},
    path::PathBuf,
};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};
use tauri::{AppHandle, Manager};

use crate::{
    database::get_sqlite_connection,
    logger::{self, write_log, LogLevel},
    page_generators::game_routes::{RouteProperties, Routes, TrainerInfo, WildEncounter},
};

pub type Wikis = HashMap<String, Wiki>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wiki {
    name: String,
    description: String,
    author: String,
    site_name: String,
    site_url: String,
    repo_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    deployment_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manifest {
    pub body: String,
    pub data: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Migration {
    pub name: String,
    pub app_version: String,
    pub execution_order: i32,
    pub sql: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OldRoutes {
    pub routes: IndexMap<String, OldRouteProperties>,
    pub encounter_areas: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OldRouteProperties {
    pub render: bool,
    pub position: i32,
    pub trainers: IndexMap<String, TrainerInfo>,
    pub wild_encounters: IndexMap<String, Vec<OldWildEncounter>>,
    pub wild_encounter_area_levels: IndexMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OldWildEncounter {
    pub id: usize,
    pub name: String,
    pub encounter_rate: f32,
    pub encounter_area: String,
    pub route: String,
    pub special_note: String,
}

impl Migration {
    pub async fn insert_migration(&self, conn: &Pool<Sqlite>) -> Result<(), String> {
        match sqlx::query(
            "INSERT INTO migrations (name, app_version, execution_order, sql) VALUES (?, ?, ?, ?)",
        )
        .bind(&self.name)
        .bind(&self.app_version)
        .bind(self.execution_order)
        .bind(&self.sql)
        .execute(conn)
        .await
        {
            Ok(_) => {}
            Err(err) => return Err(format!("Failed to insert migration: {}", err)),
        };

        Ok(())
    }

    pub async fn execute_migration(
        &self,
        wiki_path: &PathBuf,
        conn: &Pool<Sqlite>,
    ) -> Result<(), String> {
        logger::write_log(
            &wiki_path,
            logger::LogLevel::Debug,
            &format!("Executing migration: {}", self.name),
        );
        if let Err(err) = sqlx::query(&self.sql).execute(conn).await {
            return Err(err.to_string());
        }
        self.insert_migration(conn).await?;

        Ok(())
    }
}

#[tauri::command]
pub async fn check_and_run_migrations(app_handle: AppHandle) -> Result<String, String> {
    let base_path = app_handle.path().app_data_dir().unwrap();
    let resources_path = app_handle.path().resource_dir().unwrap();

    let migrations = gather_migrations(&base_path, &resources_path)?;
    //Passing down resources for one-time sprite fix. Remove later
    run_migrations(migrations, &base_path, &resources_path).await?;

    Ok("Migration Completed".to_string())
}

pub fn gather_migrations(
    base_path: &PathBuf,
    resources_path: &PathBuf,
) -> Result<Vec<Migration>, String> {
    let migrations_folder = resources_path.join("resources").join("migrations");
    if !migrations_folder.exists() || !migrations_folder.is_dir() {
        return Err("No migrations folder found".to_string());
    }

    let mut migrations: Vec<Migration> = Vec::new();

    let migrations_from_file = fs::read_dir(migrations_folder).unwrap();

    for migration in migrations_from_file {
        let migration_path = migration.unwrap().path();
        if migration_path.is_file() && migration_path.extension().unwrap() == "json" {
            logger::write_log(
                base_path,
                logger::LogLevel::Debug,
                &format!("Reading migration file: {}", migration_path.display()),
            );
            let migration_content = fs::read_to_string(migration_path).unwrap();
            let migration: Migration = match serde_json::from_str(&migration_content) {
                Ok(m) => m,
                Err(e) => {
                    println!("Error: {}", e);
                    logger::write_log(
                        base_path,
                        logger::LogLevel::MigrationError,
                        &format!("Failed to parse migration: {}", e),
                    );
                    continue;
                }
            };
            migrations.push(migration);
        }
    }

    migrations.sort_by(|a, b| a.execution_order.cmp(&b.execution_order));

    return Ok(migrations);
}

pub async fn run_migrations(
    migrations: Vec<Migration>,
    base_path: &PathBuf,
    resources_path: &PathBuf, //Passing down resources for one-time sprite fix
) -> Result<String, String> {
    let wiki_json_file_path = base_path.join("wikis.json");
    let wikis_file = match File::open(&wiki_json_file_path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to read wikis file: {}", err)),
    };
    let wikis: Wikis = match serde_json::from_reader(wikis_file) {
        Ok(wikis) => wikis,
        Err(err) => return Err(format!("Failed to parse wikis file: {}", err)),
    };

    for (wiki_name, _) in wikis.iter() {
        let wiki_path = base_path.join(wiki_name);

        if !wiki_path.exists() {
            return Err(format!("Wiki path does not exist: {:?}", wiki_path));
        }

        let sqlite_file_path = wiki_path.join(format!("{}.db", wiki_name));

        // Create backup
        if let Err(err) = std::fs::copy(
            sqlite_file_path.clone(),
            wiki_path.join(format!("{}.db.bak", wiki_name)),
        ) {
            logger::write_log(
                &wiki_path,
                logger::LogLevel::MigrationError,
                &format!("Failed to create backup: {}", err),
            );
            continue;
        };

        let conn = match get_sqlite_connection(sqlite_file_path).await {
            Ok(conn) => conn,
            Err(err) => {
                logger::write_log(
                    &wiki_path,
                    logger::LogLevel::MigrationError,
                    &format!("Failed to connect to database: {}", err),
                );
                continue;
            }
        };

        let existing_migrations = match sqlx::query_as::<_, Migration>("SELECT * FROM migrations")
            .fetch_all(&conn)
            .await
        {
            Ok(mut migrations) => {
                migrations.sort_by(|a, b| a.execution_order.cmp(&b.execution_order));
                let existing_migration_names = migrations
                    .iter()
                    .map(|m| m.name.clone())
                    .collect::<Vec<String>>();
                existing_migration_names
            }
            Err(err) => {
                if err.to_string().contains("no such table") {
                    create_migrations_table(&conn).await?;
                    let existing_migration_names = Vec::new();
                    existing_migration_names
                } else {
                    logger::write_log(
                        &wiki_path,
                        logger::LogLevel::MigrationError,
                        &format!("Failed to fetch existing migrations: {}", err),
                    );
                    continue;
                }
            }
        };

        for migration in migrations.iter() {
            // We check if the migration has already been executed
            // If it has, we skip it
            if existing_migrations.contains(&migration.name) {
                continue;
            }

            if let Err(err) = migration.execute_migration(&wiki_path, &conn).await {
                logger::write_log(
                    &wiki_path,
                    logger::LogLevel::MigrationError,
                    &format!("Failed to execute migration {}: {}", migration.name, err),
                );
                continue;
            }
        }

        // Bespoke migration for sprite updates.
        if let Err(err) = std::fs::copy(
            resources_path
                .join("resources")
                .join("generator_assets")
                .join("pokemon_sprites")
                .join("mega-diancie.png"),
            wiki_path
                .join("dist")
                .join("docs")
                .join("img")
                .join("pokemon")
                .join("mega-diancie.png"),
        ) {
            logger::write_log(
                &wiki_path,
                logger::LogLevel::MigrationError,
                &format!("Failed to update sprites: {}", err),
            );
            continue;
        };

        if let Err(err) = std::fs::copy(
            resources_path
                .join("resources")
                .join("generator_assets")
                .join("pokemon_sprites")
                .join("primal-groudon.png"),
            wiki_path
                .join("dist")
                .join("docs")
                .join("img")
                .join("pokemon")
                .join("primal-groudon.png"),
        ) {
            logger::write_log(
                &wiki_path,
                logger::LogLevel::MigrationError,
                &format!("Failed to update sprites: {}", err),
            );
            continue;
        };

        match update_route_properties(&base_path, &wiki_name).await {
            Ok(_) => (),
            Err(err) => {
                logger::write_log(
                    &wiki_path,
                    logger::LogLevel::MigrationError,
                    &format!("Failed to update route properties: {}", err),
                );
            }
        }

        logger::write_log(
            &wiki_path,
            logger::LogLevel::MigrationSuccess,
            &format!("Migrations successful"),
        );
    }

    Ok("Migrations Successful".to_string())
}

async fn update_route_properties(base_path: &PathBuf, wiki_name: &str) -> Result<(), String> {
    let routes_json_file_path = base_path.join(wiki_name).join("data").join("routes.json");
    let routes_file = match std::fs::File::open(&routes_json_file_path) {
        Ok(file) => file,
        Err(err) => {
            return Err(format!("Failed to open routes file: {}", err));
        }
    };
    let routes: OldRoutes = match serde_yaml::from_reader(routes_file) {
        Ok(routes) => routes,
        Err(err) => {
            return Err(format!("Failed to parse routes file: {}", err));
        }
    };

    let mut new_routes_properties: IndexMap<String, RouteProperties> = IndexMap::new();

    for (route_name, route) in routes.routes {
        let mut wild_encounters: Vec<WildEncounter> = Vec::new();
        for (area, encounters) in route.wild_encounters.iter() {
            for encounter in encounters.iter() {
                wild_encounters.push(WildEncounter {
                    id: encounter.id,
                    name: encounter.name.clone(),
                    encounter_area: area.clone(),
                    encounter_rate: encounter.encounter_rate,
                    route: route_name.clone(),
                    special_note: encounter.special_note.clone(),
                    route_variant: "default".to_string(),
                });
            }
        }

        let route_properties = RouteProperties {
            render: route.render,
            position: route.position,
            trainers: route.trainers,
            variants: vec!["default".to_string()],
            wild_encounters,
            wild_encounter_area_levels: route.wild_encounter_area_levels,
        };

        new_routes_properties.insert(route_name, route_properties);
    }

    let new_routes = Routes {
        routes: new_routes_properties,
        encounter_areas: routes.encounter_areas,
    };

    if let Err(err) = fs::write(
        &routes_json_file_path,
        serde_json::to_string_pretty(&new_routes).unwrap(),
    ) {
        let message = format!("{wiki_name}: Failed to update routes file: {err}");
        write_log(&base_path, LogLevel::Error, &message);
        return Err(message);
    }

    Ok(())
}

async fn create_migrations_table(conn: &Pool<Sqlite>) -> Result<(), String> {
    let migration: Migration = Migration {
        name: "create_migrations_table".to_string(),
        app_version: "1.7.5".to_string(),
        execution_order: 1,
        sql: "CREATE TABLE IF NOT EXISTS migrations (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    app_version TEXT NOT NULL,
                    execution_order INTEGER NOT NULL,
                    sql TEXT NOT NULL
                )"
        .to_string(),
    };

    if let Err(err) = sqlx::query(&migration.sql).execute(conn).await {
        return Err(err.to_string());
    }
    migration.insert_migration(conn).await?;

    Ok(())
}
