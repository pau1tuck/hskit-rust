use diesel;
use diesel::prelude::*;
use diesel::PgConnection;

pub use crate::db::character::Character;

pub use crate::db::schema::module_characters;
pub use crate::db::schema::modules;

#[derive(AsChangeset, Queryable, Identifiable, Debug, Clone, PartialEq)]
#[table_name = "modules"]
pub struct Module {
    pub id: i16,
    pub module_code: u16,
}

#[derive(Debug, Insertable)]
#[table_name = "modules"]
pub struct NewModule {
    pub module_code: u16,
}

impl Module {
    pub fn create(
        new_module: NewModule,
        connection: &PgConnection,
    ) -> QueryResult<Module> {
        diesel::insert_into(characters::table)
            .values(&new_module)
            .get_result(connection)
    }

    pub fn get_module(id: i16, connection: &PgConnection) -> QueryResult<Module> {
        modules::table.find(id).first::<Module>(connection)
    }

    pub fn get_all_modules(
        connection: &PgConnection,
    ) -> QueryResult<Vec<Module>> {
        modules::table.order(modules::id).load::<Module>(connection)
    }

    pub fn update(id: i16, module: Module, connection: &PgConnection) -> bool {
        diesel::update(modules::table.find(id))
            .set(&module)
            .execute(connection)
            .is_ok()
    }

    pub fn delete(id: i16, connection: &PgConnection) -> bool {
        diesel::delete(modules::table.find(id)).execute(connection).is_ok()
    }
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Character)]
#[belongs_to(Module)]
pub struct ModuleCharacter {
    pub id: i16,
    pub module_id: u16,
    pub character_id: u16,
}

#[derive(Debug, Insertable)]
#[table_name="module_characters"]
pub struct NewModuleCharacter {
    pub module_id: i16,
    pub character_id: u16,
}

impl ModuleCharacter {
    pub fn create(
        new_module: NewModuleCharacter,
        connection: &PgConnection,
    ) -> QueryResult<ModuleCharacter> {
        diesel::insert_into(characters::table)
            .values(&new_module)
            .get_result(connection)
    }

    pub fn read_one(id: i16, connection: &PgConnection) -> QueryResult<ModuleCharacter> {
        modules::table.find(id).first::<ModuleCharacter>(connection)
    }

    pub fn read_all(
        connection: &PgConnection,
    ) -> QueryResult<Vec<ModuleCharacter>> {
        modules::table.order(modules::id).load::<ModuleCharacter>(connection)
    }

    pub fn update(id: i16, module: ModuleCharacter, connection: &PgConnection) -> bool {
        diesel::update(modules::table.find(id))
            .set(&module)
            .execute(connection)
            .is_ok()
    }

    pub fn delete(id: i16, connection: &PgConnection) -> bool {
        diesel::delete(modules::table.find(id)).execute(connection).is_ok()
    }
}