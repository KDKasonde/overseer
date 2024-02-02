use serde::Deserialize;

#[derive(Deserialize)]
struct TimeEvent {
    date: String,
    type: String,
}

#[derive(Deserialize)]
struct WorkingSchedule {
    id: i64,
    timeEvents: Vec<TimeEvent>,
}

#[derive(Deserialize)] 
struct Exchange {
    id: i64,
    name: String,
    work_schedules: Vec<WorkingSchedule>,
}

#[derive(Deserialize)]
struct Instrument {
    added_on: String,
    cuurency_code: String,
    isin: String,
    max_open_quantity: f32,
    min_trade_quantity: f32,
    name: String,
    short_name: String,
    ticker: String,,
    type: String,
    working_schedule_id: String,
}

#[derive(Deserialize)]
struct ExchangeList {
    exchanges: Vec<Exchange>,
}

#[derive(Deserialize)]
struct InstrumentList {
    instruments: Vec<Instrument>,
}

impl Trading212 { 
    pub async fn fetch_available_exchanges(&self) -> Result<ExchangeList, reqwest::Error>{
        let client = &self.client;
        let target_url = format!("{}equity/metadata/exchanges", self.base_url );

        let res = client
            .get(target_url)
            .send()
            .await;
        
        let output = match res {
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
        return output
    }
    pub async fn fetch_available_instruments(&self) -> Result<InstrumentList, reqwest::Error> {
        let client = &self.client;
        let target_url = format!("{}equity/metadata/instruments", self.base_url );

        let res = client
            .get(target_url)
            .send()
            .await;
        
        let output = match res {
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
        return output
    }
}
