use diesel::prelude::*;

pub mod model;
pub mod schema;

pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations = diesel_migrations::embed_migrations!();

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::upsert::excluded;
    use model::*;
    use schema::media_archive::*;

    #[test]
    fn error() {
        let connection = SqliteConnection::establish("./sqlite.db").unwrap();

        diesel_migrations::MigrationHarness::run_pending_migrations(&mut connection, MIGRATIONS).unwrap();

        let bulk_values: Vec<InsMedia> = vec![InsMedia::new("testid", "testprovider", "testtitle")];

        diesel::insert_into(schema::media_archive::table)
            .values(&bulk_values)
            .on_conflict((media_id, provider))
            .do_update()
            .set(title.eq(excluded(title)))
            .execute(&mut connection).unwrap();

        println!("Finished");
    }
}
