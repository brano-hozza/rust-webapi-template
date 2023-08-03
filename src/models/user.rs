use crate::utils::error::AppError;
use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::backend::Backend;
use diesel::dsl::{AsSelect, Eq, Filter, Select};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::utils::hasher;

#[derive(Identifiable, Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

type All<DB> = Select<users::table, AsSelect<User, DB>>;
type WithUsername<T> = Eq<users::username, T>;
type ByUsername<DB, T> = Filter<All<DB>, WithUsername<T>>;

impl User {
    fn all<DB>() -> All<DB>
        where
            DB: Backend,
    {
        users::table.select(User::as_select())
    }

    pub fn with_username(username: &str) -> WithUsername<&str> {
        users::username.eq(username)
    }

    pub fn by_username<DB>(username: &str) -> ByUsername<DB, &str>
        where
            DB: Backend,
    {
        Self::all().filter(Self::with_username(username))
    }

}

impl User {
    pub fn find(conn: &mut PgConnection, id: Uuid) -> Result<Self, AppError> {
        let t = users::table.find(id);
        let user = t.first(conn)?;
        Ok(user)
    }

    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<Self>, AppError> {
        let t = users::table.select(User::as_select());
        let users = t.get_results(conn).ok().unwrap();
        Ok(users)
    }

    pub fn update(
        conn: &mut PgConnection,
        user_id: Uuid,
        changeset: UpdateUser,
    ) -> Result<Self, AppError> {
        let target = users::table.find(user_id);
        let user = diesel::update(target)
            .set(changeset)
            .get_result::<User>(conn)?;
        Ok(user)
    }

    pub fn signup<'a>(
        conn: &mut PgConnection,
        email: &'a str,
        username: &'a str,
        password: &'a str,
    ) -> Result<User, AppError> {
        use diesel::prelude::*;
        let hashed_password = hasher::hash_password(password)?;

        let record = SignupUser {
            email,
            username,
            password: &hashed_password,
        };

        let user = diesel::insert_into(users::table)
            .values(&record)
            .get_result::<User>(conn)?;

        Ok(user)
    }
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = users)]
pub struct SignupUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(AsChangeset, Debug, Deserialize, Clone)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}