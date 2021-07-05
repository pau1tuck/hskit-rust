table! {
    use diesel::sql_types::*;
    use crate::db::enums::*;

    /// Representation of the `heroes` table.
    ///
    /// (Automatically generated by Diesel.)
    heroes (id) {
        /// The `id` column of the `heroes` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `name` column of the `heroes` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
        /// The `appears_in` column of the `heroes` table.
        ///
        /// Its SQL type is `Array<EpisodeMapping>`.
        ///
        /// (Automatically generated by Diesel.)
        appears_in -> Array<EpisodeMapping>,
        /// The `home_planet` column of the `heroes` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        home_planet -> Varchar,
    }
}
