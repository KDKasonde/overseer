use super::traits::ReadableSecurity;

pub struct Account {
    pub vendor: String,
    pub blocked: f32,
    pub free: f32,
    pub total_funds: f32,
    pub invested: f32,
    pub ppl: f32,
    pub total: f32
}

pub struct Position {
    pub security_id: String,
    pub security_name: String,
    pub security_name_subtext: String,
    pub total_value: f32,
    pub total_cost: f32,
    pub current_price: f32,
    pub ppl: f32,
    pub ppl_as_perc: f32,
    pub quantity: f32
}

pub struct HistoricalTransaction {
    pub security_name: String,
    pub security_name_subtext: String,
    pub date: String,
    pub unit_cost: f32,
    pub quantity: f32,
    pub cost: f32,
    pub transaction_type: String,
}

impl ReadableSecurity for Position {
    fn get_security_id(&self) -> String {
        self.security_id.to_owned()
    }

    fn get_security_name(&self) -> String {
        self.security_name.to_owned()
    }
}

