#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    Jinja(#[from] minijinja::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("not found")]
    NotFound,

    #[error(transparent)]
    Base2(#[from] mm_base2::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
