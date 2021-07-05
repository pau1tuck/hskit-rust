use diesel;
use diesel::prelude::*;
use diesel::PgConnection;

pub use crate::db::character::Character;

pub use crate::db::schema::module_characters;
pub use crate::db::schema::modules;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Character)]
#[belongs_to(Module)]
pub struct ModuleCharacter {
    pub id: i16,
    pub module_id: i16,
    pub character_id: i16,
}

#[derive(Debug, Insertable)]
#[table_name="module_characters"]
pub struct NewModuleCharacter {
    pub module_id: i16,
    pub character_id: i16,
}

#[derive(AsChangeset, Queryable, Identifiable, Debug, Clone, PartialEq)]
#[table_name = "modules"]
pub struct Module {
    pub id: i16,
    pub module_code: i16,
}