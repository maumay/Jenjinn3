use std::env;
use std::error::Error;

/// Keys for required environment variables
const MYOPIC_EXPECTED_HALF_MOVES: &'static str = "MYOPIC_EXPECTED_HALF_MOVES";
// Lambda variables
const MYOPIC_FUNCTION_REGION: &'static str = "MYOPIC_FUNCTION_REGION";
const MYOPIC_FUNCTION_NAME: &'static str = "MYOPIC_FUNCTION_NAME";
// Lichess variables
const MYOPIC_LICHESS_AUTH_TOKEN: &'static str = "MYOPIC_LICHESS_AUTH_TOKEN";
const MYOPIC_LICHESS_BOT_ID: &'static str = "MYOPIC_LICHESS_BOT_ID";
// Opening table variables
const MYOPIC_OPENING_TABLE_NAME: &'static str = "MYOPIC_OPENING_TABLE_NAME";
const MYOPIC_OPENING_TABLE_REGION: &'static str = "MYOPIC_OPENING_TABLE_REGION";
const MYOPIC_OPENING_TABLE_POSITION_KEY: &'static str = "MYOPIC_OPENING_TABLE_POSITION_KEY";
const MYOPIC_OPENING_TABLE_MOVE_KEY: &'static str = "MYOPIC_OPENING_TABLE_MOVE_KEY";
// Timing variables
const MYOPIC_MIN_INITIAL_TIME_SECS: &'static str = "MYOPIC_MIN_INITIAL_TIME_SECS";
const MYOPIC_MAX_INITIAL_TIME_SECS: &'static str = "MYOPIC_MAX_INITIAL_TIME_SECS";
const MYOPIC_MIN_INCREMENT_SECS: &'static str = "MYOPIC_MIN_INCREMENT_SECS";
const MYOPIC_MAX_INCREMENT_SECS: &'static str = "MYOPIC_MAX_INCREMENT_SECS";
const MYOPIC_MAX_LAMBDA_DURATION_MINS: &'static str = "MYOPIC_MAX_LAMBDA_DURATION_MINS";
const MYOPIC_INCREMENT_ALLOWANCE_MINS: &'static str = "MYOPIC_INCREMENT_ALLOWANCE_MINS";
const MYOPIC_RETRY_WAIT_DURATION_SECS: &'static str = "MYOPIC_RETRY_WAIT_DURATION_SECS";

#[derive(Debug, Clone)]
pub struct ApplicationParameters {
    pub expected_half_moves: u32,
    pub function_region: String,
    pub function_name: String,
    pub lichess_auth_token: String,
    pub lichess_bot_id: String,
    pub opening_table_name: String,
    pub opening_table_region: String,
    pub opening_table_position_key: String,
    pub opening_table_move_key: String,
    pub min_initial_time_secs: u32,
    pub max_initial_time_secs: u32,
    pub min_increment_secs: u32,
    pub max_increment_secs: u32,
    pub max_lambda_duration_mins: u8,
    pub increment_allowance_mins: u8,
    pub retry_wait_duration_secs: u64,
}

impl ApplicationParameters {
    pub fn load() -> Result<ApplicationParameters, Box<dyn Error>> {
        Ok(ApplicationParameters {
            expected_half_moves: env::var(MYOPIC_EXPECTED_HALF_MOVES)?.parse()?,
            function_region: env::var(MYOPIC_FUNCTION_REGION)?,
            function_name: env::var(MYOPIC_FUNCTION_NAME)?,
            lichess_auth_token: env::var(MYOPIC_LICHESS_AUTH_TOKEN)?,
            lichess_bot_id: env::var(MYOPIC_LICHESS_BOT_ID)?,
            opening_table_name: env::var(MYOPIC_OPENING_TABLE_NAME)?,
            opening_table_region: env::var(MYOPIC_OPENING_TABLE_REGION)?,
            opening_table_position_key: env::var(MYOPIC_OPENING_TABLE_POSITION_KEY)?,
            opening_table_move_key: env::var(MYOPIC_OPENING_TABLE_MOVE_KEY)?,
            min_initial_time_secs: env::var(MYOPIC_MIN_INITIAL_TIME_SECS)?.parse()?,
            max_initial_time_secs: env::var(MYOPIC_MAX_INITIAL_TIME_SECS)?.parse()?,
            min_increment_secs: env::var(MYOPIC_MIN_INCREMENT_SECS)?.parse()?,
            max_increment_secs: env::var(MYOPIC_MAX_INCREMENT_SECS)?.parse()?,
            max_lambda_duration_mins: env::var(MYOPIC_MAX_LAMBDA_DURATION_MINS)?.parse()?,
            increment_allowance_mins: env::var(MYOPIC_INCREMENT_ALLOWANCE_MINS)?.parse()?,
            retry_wait_duration_secs: env::var(MYOPIC_RETRY_WAIT_DURATION_SECS)?.parse()?,
        })
    }

    pub fn to_lambda_invocation_payload(
        &self,
        game_id: String,
        depth: u8,
    ) -> Result<String, Box<dyn Error>> {
        serde_json::to_string(&PlayGameEvent {
            expected_half_moves: self.expected_half_moves,
            function_depth_remaining: depth,
            function_region: self.function_region.clone(),
            function_name: self.function_name.clone(),
            lichess_game_id: game_id,
            lichess_auth_token: self.lichess_auth_token.clone(),
            lichess_bot_id: self.lichess_bot_id.clone(),
            opening_table_name: self.opening_table_name.clone(),
            opening_table_region: self.opening_table_region.clone(),
            opening_table_position_key: self.opening_table_position_key.clone(),
            opening_table_move_key: self.opening_table_move_key.clone(),
        })
        .map_err(|error| Box::new(error) as Box<dyn Error>)
    }
}

/// The input payload of this lambda
#[derive(Serialize, Deserialize, Clone)]
struct PlayGameEvent {
    /// The current call depth of the lambda invokation
    #[serde(rename = "functionDepthRemaining")]
    function_depth_remaining: u8,
    /// The region this lambda is deployed in
    #[serde(rename = "functionRegion")]
    function_region: String,
    /// The name of this lambda function
    #[serde(rename = "functionName")]
    function_name: String,
    /// The lichess game id this lambda will participate in
    #[serde(rename = "lichessGameId")]
    lichess_game_id: String,
    /// An auth token for the lichess bot this lambda will play as
    #[serde(rename = "lichessAuthToken")]
    lichess_auth_token: String,
    /// The id of the lichess bot this lambda will play as
    #[serde(rename = "lichessBotId")]
    lichess_bot_id: String,
    /// The name of the dynamodb table used to store opening positions
    #[serde(rename = "openingTableName")]
    opening_table_name: String,
    /// The region in which the opening table is deployed
    #[serde(rename = "openingTableRegion")]
    opening_table_region: String,
    /// The name of the position key used as a pk in the opening table
    #[serde(rename = "openingTablePositionKey")]
    opening_table_position_key: String,
    /// The name of the move key used in the opening table
    #[serde(rename = "openingTableMoveKey")]
    opening_table_move_key: String,
    /// How many half moves we expect the game to last for
    #[serde(rename = "expectedHalfMoves")]
    expected_half_moves: u32,
}
