use serde::{ Serialize, Deserialize };

#[derive(Deserialize, Serialize, Debug, Clone,Copy, PartialEq, Eq)]
pub enum AppState {
    Landing,
    ParentalCheck,
    MainFeed
}
impl Default for AppState {
    fn default() -> Self {
        Self::Landing
    }
}