use super::Trading212;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename = "camelCase")]
struct TimeEvent {
    date: String,
    item_type: String,
}

#[derive(Deserialize)]
#[serde(rename = "camelCase")]
struct WorkingSchedule {
    id: i64,
    time_events: Vec<TimeEvent>,
}

#[derive(Deserialize)] 
#[serde(rename = "camelCase")]
struct Exchange {
    id: i64,
    name: String,
    work_schedules: Vec<WorkingSchedule>,
}

#[derive(Deserialize)]
#[serde(rename = "camelCase")]
struct Instrument {
    added_on: String,
    cuurency_code: String,
    isin: String,
    max_open_quantity: f32,
    min_trade_quantity: f32,
    name: String,
    short_name: String,
    ticker: String,
    item_type: String,
    working_schedule_id: String,
}

#[derive(Deserialize)]
#[serde(rename = "camelCase")]
struct ExchangeList {
    exchanges: Vec<Exchange>,
}

#[derive(Deserialize)]
#[serde(rename = "camelCase")]
struct InstrumentList {
    instruments: Vec<Instrument>,
}

impl Trading212 { 
    pub async fn fetch_available_exchanges(&self) -> ExchangeList {
        let client = &self.client;
        let target_url = format!("{}equity/metadata/exchanges", self.base_url );

        let res = client
            .get(target_url)
            .send()
            .await;
        
        let res = match res {
            Ok(response) => { 
                response
                    .json::<ExchangeList>()
                    .await
            },
            Err(error)  => {
                // This should not panic unless there is something wrong with auth, the url or the
                // headers.
                panic!("Response was not okay! Received the following error: \n\t{}", error);
            }
        }; 
        
        let output = match res {
            Ok(response) => { 
                response
            },
            Err(error)  => {
                panic!("Derserialization failed, error: \n\t{}", error);
            }
        }; 
        
        return output
    }
    
    pub async fn fetch_available_instruments(&self) -> InstrumentList {
        let client = &self.client;
        let target_url = format!("{}equity/metadata/instruments", self.base_url );

        let res = client
            .get(target_url)
            .send()
            .await;
        
        let res = match res {
            Ok(response) => { 
                response
                    .json::<InstrumentList>()
                    .await
            },
            Err(error)  => {
                // This should not panic unless there is something wrong with auth, the url or the
                // headers.
                panic!("Response was not okay! Received the following error: \n\t{}", error);
            }
        }; 
        
        let output = match res {
            Ok(response) => { 
                response
            },
            Err(error)  => {
                panic!("Derserialization failed, error: \n\t{}", error);
            }
        }; 
        
        return output
    }
}
