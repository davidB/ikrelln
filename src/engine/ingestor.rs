use std;
use serde;
use actix::*;
use chrono;


#[derive(Deserialize, Serialize, Debug, Clone)]
enum Status {
    SUCCESS,
    FAILURE,
    SKIPPED,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TestResult {
    test_name: String,
    result: Status,
    #[serde(deserialize_with = "deserialize_duration")] duration: std::time::Duration,
}

use serde::de::{self, Deserialize, MapAccess, Visitor};
fn deserialize_duration<'de, D>(
    deserializer: D,
) -> std::result::Result<std::time::Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct IntOrStruct(std::marker::PhantomData<fn() -> std::time::Duration>);

    impl<'de> Visitor<'de> for IntOrStruct {
        type Value = std::time::Duration;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("int or map")
        }

        fn visit_u64<E>(self, value: u64) -> Result<std::time::Duration, E>
        where
            E: de::Error,
        {
            Ok(std::time::Duration::new(value, 0))
        }

        fn visit_map<M>(self, visitor: M) -> Result<std::time::Duration, M::Error>
        where
            M: MapAccess<'de>,
        {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(IntOrStruct(std::marker::PhantomData))
}

use db::schema::ingest;
#[derive(Debug, Insertable)]
#[table_name = "ingest"]
pub struct IngestEventDb {
    id: String,
    created_at: String,
    processed_at: Option<String>,
}
impl From<IngestEvents> for IngestEventDb {
    fn from(ie: IngestEvents) -> IngestEventDb {
        IngestEventDb {
            id: ie.ingest_id.to_string(),
            created_at: ie.created_at.to_rfc2822(),
            processed_at: ie.processed_at.map(|date| date.to_rfc2822()),
        }
    }
}

#[derive(Debug)]
pub struct IngestEvents {
    pub ingest_id: super::IngestId,
    events: Vec<TestResult>,
    created_at: chrono::DateTime<chrono::UTC>,
    processed_at: Option<chrono::DateTime<chrono::UTC>>,
}
impl IngestEvents {
    pub fn new(events: Vec<TestResult>) -> IngestEvents {
        IngestEvents {
            ingest_id: super::IngestId::new(),
            events: events,
            created_at: chrono::UTC::now(),
            processed_at: None,
        }
    }
}

impl ResponseType for IngestEvents {
    type Item = ();
    type Error = ();
}

pub struct Ingestor(pub SyncAddress<::db::DbExecutor>);

impl Actor for Ingestor {
    type Context = Context<Self>;
}

impl Handler<IngestEvents> for Ingestor {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: IngestEvents, _ctx: &mut Context<Self>) -> Self::Result {
        info!("{:?}", msg);
        self.0.send(msg);
        Ok(())
    }
}
