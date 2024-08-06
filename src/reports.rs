use serde::Deserialize;

#[derive(Deserialize, Debug)]

pub struct SbAdGroupReport {
    pub campaignId: String,
    pub adGroupId: String,
    pub impressions: i32,
    pub clicks: i32,
    pub cost: f64,
    pub conversions: i32,
    pub revenue: f64,
}
