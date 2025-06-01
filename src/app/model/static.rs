// The definition of tables with a static content

use diesel::Queryable;

#[derive(Queryable, Debug, Clone)]
#[diesel(table_name = crate::schema::player_class)]
pub struct PlayerClass {
    pub class_id: i32,
    pub class_name: String,
    pub class_spec_one_name: Option<String>,
    pub class_spec_two_name: Option<String>,
    pub class_spec_tree_name: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Queryable, Debug, Clone)]
#[diesel(table_name = crate::schema::player_rank_table)]
pub struct PlayerRankTable {
    pub id: i32,
    pub valor: i32,
    pub min_level: i32,
    pub rank_name_EN: String,
    pub rank_name_FR: String,
    pub rank_name_UA: String,
    pub rank_pic_url_EN: Option<String>,
    pub rank_pic_url_FR: Option<String>,
}

#[derive(Queryable, Debug, Clone)]
#[diesel(table_name = crate::schema::player_experience_table)]
pub struct PlayerExperienceTable {
    pub exp: i32,
    pub up: i32,
    pub level: i32,
    pub attrs: i32,
    pub money: i32,
}
