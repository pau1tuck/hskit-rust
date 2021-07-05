#[derive(Debug, PartialEq, DbEnum, Clone, GraphQLEnum)]
#[PgType = "episode_enum"]
pub enum Episode {
    NewHope,
    Empire,
    Jedi,
}
#[derive(Debug, PartialEq, DbEnum, Clone, GraphQLEnum)]
#[PgType = "level_enum"]
pub enum Level {
    Toilet,
    Water,
}