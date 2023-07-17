use once_cell::sync::Lazy;

pub const VALID_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/116.0";
pub const DISNEY_ROOT_URL: &str = "https://disneyworld.disney.go.com";
pub static SPACE_URL: Lazy<String> = Lazy::new(|| format!("{}/finder/api/v1/explorer-service/dining-availability/%7B9DDDAEF3-9DAC-46B6-B55B-A12FC04588DF%7D/wdw/19634138;entityType=restaurant/table-service/2/2023-09-13/?mealPeriod=80000717", DISNEY_ROOT_URL));