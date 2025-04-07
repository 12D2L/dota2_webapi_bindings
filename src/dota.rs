//==============================================================================
//IEconDOTA2_<ID>
//==============================================================================

//! I have by default use Json instead of XML cause it is more popular and easy to work with, and you do not need ti use
//! them directly!!
//!
//! Almost all of the structs have `localized_name : Option<String>` as parameter, this will always
//! return `None` unless you use `language()` parameter
//! **Note**: I recommend using `language` cause it gives names like "clarity" instead of "item_clarity"

//==============================================================================
//IEconDOTA2_570
//==============================================================================

pub mod get_heroes {
    #[derive(Deserialize, Debug)]
    pub struct GetHeroesResult {
        pub result: GetHeroes,
    }

    #[derive(Deserialize, Debug)]
    pub struct GetHeroes {
        pub heroes: Vec<Hero>,
        pub count: usize,
        pub status: usize,
    }

    #[derive(Deserialize, Debug)]
    pub struct Hero {
        pub name: String,
        pub id: usize,
        pub localized_name: Option<String>,
    }
}

pub mod get_game_items {
    #[derive(Deserialize, Debug)]
    pub struct GetGameItemsResult {
        pub result: GetGameItems,
    }

    #[derive(Deserialize, Debug)]
    pub struct GetGameItems {
        pub items: Vec<Item>,
        pub status: usize,
    }

    #[derive(Deserialize, Debug)]
    pub struct Item {
        pub id: usize,
        pub name: String,
        pub cost: usize,
        pub secret_shop: usize,
        pub side_shop: usize,
        pub recipe: usize,
        pub localized_name: Option<String>,
    }
}

pub mod get_rarities {
    #[derive(Deserialize, Debug)]
    pub struct GetRaritiesResult {
        pub result: GetRarities,
    }

    #[derive(Deserialize, Debug)]
    pub struct GetRarities {
        pub count: usize,
        pub rarities: Vec<Rarity>,
        pub status: usize,
    }

    #[derive(Deserialize, Debug)]
    pub struct Rarity {
        pub name: String,
        pub id: usize,
        pub order: usize,
        pub color: String,
        pub localized_name: Option<String>,
    }
}

pub mod get_tournament_prize_pool {
    #[derive(Deserialize, Debug)]
    pub struct GetTournamentPrizePoolResult {
        pub result: GetTournamentPrizePool,
    }

    #[derive(Deserialize, Debug)]
    pub struct GetTournamentPrizePool {
        pub prize_pool: usize,
        pub league_id: usize,
        pub status: usize,
    }
}

//==============================================================================
//IDOTA2Match_205790
//==============================================================================

pub mod get_league_listing {
    #[derive(Deserialize, Debug)]
    pub struct GetLeagueListingResult {
        pub result: GetLeagueListing,
    }

    #[derive(Deserialize, Debug)]
    pub struct GetLeagueListing {
        pub leagues: Vec<League>,
    }

    #[derive(Deserialize, Debug)]
    pub struct League {
        pub name: String,
        #[serde(rename = "leagueid")]
        pub league_id: usize,
        pub description: Option<String>,
        pub tournament_url: String,
        #[serde(rename = "itemdef")]
        pub item_def: usize,
    }
}

//==============================================================================
//IDOTA2Match_570
//==============================================================================

pub mod get_live_league_games {

    use serde::de::{self, Error as _, MapAccess, Visitor};
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct GetLiveLeagueGamesResult {
        pub result: GetLiveLeagueGames,
    }

    #[derive(Deserialize, Debug)]
    pub struct GetLiveLeagueGames {
        pub games: Vec<Game>,
        pub status: usize,
    }

    #[derive(Deserialize, Debug)]
    pub struct Game {
        pub players: Vec<Player>,
        pub radiant_team: Option<RadiantTeam>,
        pub dire_team: Option<DireTeam>,
        pub lobby_id: usize,
        pub match_id: usize,
        pub spectators: usize,
        pub league_id: usize,
        pub league_node_id: usize,
        pub stream_delay_s: usize,
        pub radiant_series_wins: usize,
        pub dire_series_wins: usize,
        pub series_type: usize,
        pub scoreboard: Option<Scoreboard>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Player {
        pub account_id: usize,
        pub name: String,
        pub hero_id: usize,
        pub team: usize,
    }

    #[derive(Deserialize, Debug)]
    pub struct RadiantTeam {
        pub team_name: String,
        pub team_id: usize,
        pub team_logo: usize,
        pub complete: bool,
    }

    #[derive(Deserialize, Debug)]
    pub struct DireTeam {
        pub team_name: String,
        pub team_id: usize,
        pub team_logo: usize,
        pub complete: bool,
    }

    #[derive(Deserialize, Debug)]
    pub struct Scoreboard {
        pub duration: f64,
        pub roshan_respawn_timer: usize,
        pub radiant: Ancient,
        pub dire: Ancient,
    }

    #[derive(Deserialize, Debug)]
    pub struct Ancient {
        pub score: usize,
        pub tower_state: usize,
        pub barracks_state: usize,
        pub picks: Option<Vec<HeroId>>,
        pub bans: Option<Vec<HeroId>>,
        pub players: Vec<PlayerDetailed>,
        #[serde(flatten)]
        pub abilities: Abilities,
    }

    #[derive(Debug)]
    pub struct Abilities(pub Vec<Ability>);

    #[derive(Debug, Deserialize)]
    pub struct Ability {
        pub ability_level: usize,
        pub ability_id: usize,
    }

    impl<'de> Deserialize<'de> for Abilities {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct MyVisitor;

            impl<'d> Visitor<'d> for MyVisitor {
                type Value = Vec<Ability>;

                fn expecting(
                    &self,
                    f: &mut std::fmt::Formatter<'_>,
                ) -> Result<(), std::fmt::Error> {
                    f.write_str("a map of abilities")
                }

                fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
                where
                    M: MapAccess<'d>,
                {
                    let mut abilities = Vec::new();
                    while let Some((key, mut value)) = access.next_entry()? {
                        if key == "abilities" {
                            abilities.append(&mut value);
                        } else {
                            return Err(M::Error::unknown_field(key, &["abilities"]));
                        }
                    }
                    Ok(abilities)
                }
            }
            Ok(Abilities(deserializer.deserialize_map(MyVisitor)?))
        }
    }

    #[derive(Deserialize, Debug)]
    pub struct HeroId {
        pub hero_id: usize,
    }

    #[derive(Deserialize, Debug)]
    pub struct PlayerDetailed {
        pub player_slot: usize,
        pub account_id: usize,
        pub hero_id: usize,
        pub kills: usize,
        pub death: usize,
        pub assists: usize,
        pub last_hits: usize,
        pub denies: usize,
        pub gold: usize,
        pub level: usize,
        pub gold_per_min: usize,
        pub xp_per_min: usize,
        pub ultimate_state: usize,
        pub ultimate_cooldown: usize,
        pub item0: i32,
        pub item1: i32,
        pub item2: i32,
        pub item3: i32,
        pub item4: i32,
        pub item5: i32,
        pub respawn_timer: usize,
        pub position_x: f64,
        pub position_y: f64,
        pub net_worth: usize,
    }
}

pub mod get_top_live_game {
    #[derive(Deserialize, Debug)]
    pub struct GetTopLiveGame {
        pub game_list: Vec<GameList>,
    }

    #[derive(Deserialize, Debug)]
    pub struct GameList {
        pub activate_time: usize,
        pub deactivate_time: usize,
        pub lobby_id: usize,
        pub league_id: usize,
        pub lobby_type: usize,
        pub game_type: Option<usize>,
        pub delay: usize,
        pub spectators: usize,
        pub game_mode: usize,
        pub average_mmr: usize,
        pub match_id: usize,
        pub series_id: usize,
        pub team_name_radiant: Option<String>,
        pub team_name_dire: Option<String>,
        pub sort_score: usize,
        pub last_update_time: usize,
        pub radiant_lead: isize,
        pub radiant_score: usize,
        pub dire_score: usize,
        pub players: Option<Vec<Player>>,
        pub building_state: usize,
        pub weekend_tourney_tournament_id: Option<usize>,
        pub weekend_tourney_division: Option<usize>,
        pub weekend_tourney_skill_level: Option<usize>,
        pub weekend_tourney_bracket_round: Option<usize>,
        pub custom_game_difficulty: usize,
    }

    #[derive(Deserialize, Debug)]
    pub struct Player {
        pub account_id: usize,
        pub hero_id: usize,
    }
}
