use crate::error::{BoxDynError, Error};

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum MigrateError {
    #[error("could not run migration {0}")]
    Execute(i64, #[source] Error),

    #[error("while resolving migrations: {0}")]
    Source(#[source] BoxDynError),

    #[error("migration {0} was previously applied but is missing in the resolved migrations")]
    VersionMissing(i64),

    #[error("migration {0} was previously applied but has been modified")]
    VersionMismatch(i64),

    #[error("cannot mix reversible migrations with simple migrations. All migrations should be reversible or simple migrations")]
    InvalidMixReversibleAndSimple,

    // NOTE: this will only happen with a database that does not have transactional DDL (.e.g, MySQL or Oracle)
    #[error(
        "migration {0} is partially applied; fix and remove row from `_sqlx_migrations` table"
    )]
    Dirty(i64),

    #[error("unable to acquire a connection to the database")]
    AcquireConnection(#[source] Error),

    #[error("an operation on the migration metadata table (_sqlx_migrations) failed")]
    AccessMigrationMetadata(#[source] Error),
}

pub type MigrateResult<T> = std::result::Result<T, MigrateError>;
