use diesel;
use diesel::prelude::*;
use diesel::PgConnection;

pub use crate::db::schema::characters;

#[derive(AsChangeset, Queryable, Identifiable, Debug, Clone, PartialEq)]
#[table_name = "characters"]
pub struct Character {
    pub id: i16,
    pub level: i8,
    pub simplified: String,
    pub traditional: String,
    pub pinyin: String,
    pub english: String,
}

#[derive(Debug, Insertable)]
#[table_name = "characters"]
pub struct NewCharacter {
    pub level: i8,
    pub simplified: String,
    pub traditional: Option<String>,
    pub pinyin: String,
    pub english: String,
}

impl Character {
    pub fn create(
        new_character: NewCharacter,
        connection: &PgConnection,
    ) -> QueryResult<Character> {
        diesel::insert_into(characters::table)
            .values(&new_character)
            .get_result(connection)
    }

    pub fn get_character(id: i32, connection: &PgConnection) -> QueryResult<Character> {
        characters::table.find(id).first::<Character>(connection)
    }

    pub fn get_all_characters(
        connection: &PgConnection,
    ) -> QueryResult<Vec<Character>> {
        characters::table.order(characters::id).load::<Character>(connection)
    }

    pub fn update(id: i32, hero: Character, connection: &PgConnection) -> bool {
        diesel::update(characters::table.find(id))
            .set(&character)
            .execute(connection)
            .is_ok()
    }

    pub fn delete(id: i32, connection: &PgConnection) -> bool {
        diesel::delete(characters::table.find(id)).execute(connection).is_ok()
    }
}
