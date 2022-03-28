#[cfg(feature = "tls")]
pub mod client;
#[cfg(feature = "wasm")]
pub mod wasm_client;

pub mod inference {
    include!(concat!(env!("OUT_DIR"), "/inference.rs"));
}

// The prost_types::Timestamp type does not have serde::Serialize/Deserialize traits implemented.
// Here we provide them.
pub mod serde_timestamp {
    use prost_types::Timestamp;
    use serde::de::Deserializer;
    use serde::ser::Serializer;
    use serde::{Deserialize, Serialize};

    // This is a version of prost_types::Timestamp with the Serde traits derived.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProtoTimestamp {
        pub seconds: i64,
        pub nanos: i32,
    }

    pub fn serialize<S>(maybe_ts: &Option<Timestamp>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        maybe_ts
            .as_ref()
            .map(|ts| ProtoTimestamp {
                seconds: ts.seconds,
                nanos: ts.nanos,
            })
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Timestamp>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let maybe_pts = Option::<ProtoTimestamp>::deserialize(deserializer)?;
        Ok(maybe_pts.map(|pts| Timestamp {
            seconds: pts.seconds,
            nanos: pts.nanos,
        }))
    }
}

// The prost_types::Duration type does not have serde::Serialize/Deserialize traits implemented.
// Here we provide them.
pub mod serde_duration {
    use prost_types::Duration;
    use serde::de::Deserializer;
    use serde::ser::Serializer;
    use serde::{Deserialize, Serialize};

    // This is a version of prost_types::Duration with the Serde traits derived.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProtoDuration {
        pub seconds: i64,
        pub nanos: i32,
    }

    impl From<&Duration> for ProtoDuration {
        fn from(d: &Duration) -> Self {
            Self {
                seconds: d.seconds,
                nanos: d.nanos,
            }
        }
    }

    impl From<Duration> for ProtoDuration {
        fn from(d: Duration) -> Self {
            (&d).into()
        }
    }

    impl From<&ProtoDuration> for Duration {
        fn from(pd: &ProtoDuration) -> Self {
            Self {
                seconds: pd.seconds,
                nanos: pd.nanos,
            }
        }
    }

    impl From<ProtoDuration> for Duration {
        fn from(pd: ProtoDuration) -> Self {
            (&pd).into()
        }
    }

    pub fn serialize<S>(maybe_duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        maybe_duration
            .as_ref()
            .map(|d| ProtoDuration::from(d))
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let maybe_duration = Option::<ProtoDuration>::deserialize(deserializer)?;
        Ok(maybe_duration.map(|d| d.into()))
    }
}

// The prost_types::Duration type does not have serde::Serialize/Deserialize traits implemented.
// Here we provide them.
pub mod serde_duration_vec {
    use prost_types::Duration;
    use serde::de::Deserializer;
    use serde::ser::Serializer;
    use serde::Deserialize;

    use super::serde_duration::ProtoDuration;

    pub fn serialize<S>(durations: &Vec<Duration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(durations.into_iter().map(|d| ProtoDuration::from(d)))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Duration>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let durations = Vec::<ProtoDuration>::deserialize(deserializer)?;
        Ok(durations.into_iter().map(|pd| pd.into()).collect())
    }
}

// The prost_types::FieldMask type does not have serde::Serialize/Deserialize traits implemented.
// Here we provide them.
pub mod serde_fieldmask {
    use prost_types::FieldMask;
    use serde::de::Deserializer;
    use serde::ser::Serializer;
    use serde::{Deserialize, Serialize};

    // This is a version of prost_types::FieldMask with the Serde traits derived.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProtoFieldMask {
        paths: Vec<String>,
    }

    pub fn serialize<S>(maybe_fm: &Option<FieldMask>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        maybe_fm
            .as_ref()
            .map(|fm| ProtoFieldMask {
                paths: fm.paths.clone(),
            })
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<FieldMask>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let maybe_pfm = Option::<ProtoFieldMask>::deserialize(deserializer)?;
        Ok(maybe_pfm.map(|pfm| FieldMask { paths: pfm.paths }))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_serde_timestamp() {
        use crate::serde_timestamp;
        use serde::{Deserialize, Serialize};
        use serde_json;

        #[derive(Serialize, Deserialize)]
        struct MyMessage {
            #[serde(with = "serde_timestamp")]
            ts: Option<prost_types::Timestamp>,
        }

        let ts = prost_types::Timestamp {
            seconds: 1234,
            nanos: 5678,
        };
        let mm = MyMessage { ts: Some(ts) };
        let result = serde_json::to_string(&mm).expect("Unable to serialize MyMessage");
        assert_eq!(result, "{\"ts\":{\"seconds\":1234,\"nanos\":5678}}");
    }

    #[test]
    fn test_serde_duration() {
        use crate::serde_duration;
        use serde::{Deserialize, Serialize};
        use serde_json;

        #[derive(Serialize, Deserialize)]
        struct MyMessage {
            #[serde(with = "serde_duration")]
            duration: Option<prost_types::Duration>,
        }

        let duration = prost_types::Duration {
            seconds: 1234,
            nanos: 5678,
        };
        let mm = MyMessage {
            duration: Some(duration),
        };
        let result = serde_json::to_string(&mm).expect("Unable to serialize MyMessage");
        assert_eq!(result, "{\"duration\":{\"seconds\":1234,\"nanos\":5678}}");

        let deserialized: MyMessage =
            serde_json::from_str(&result).expect("Unable to deserialize MyMessage");

        assert_eq!(
            deserialized.duration.as_ref().unwrap().seconds,
            mm.duration.as_ref().unwrap().seconds
        );
        assert_eq!(
            deserialized.duration.as_ref().unwrap().nanos,
            mm.duration.as_ref().unwrap().nanos
        );
    }

    #[test]
    fn test_serde_duration_vec() {
        use crate::serde_duration_vec;
        use serde::{Deserialize, Serialize};
        use serde_json;

        #[derive(Serialize, Deserialize)]
        struct MyMessage {
            #[serde(with = "serde_duration_vec")]
            durations: Vec<prost_types::Duration>,
        }

        let duration = prost_types::Duration {
            seconds: 1234,
            nanos: 5678,
        };
        let mm = MyMessage {
            durations: vec![duration],
        };
        let result = serde_json::to_string(&mm).expect("Unable to serialize MyMessage");
        assert_eq!(
            result,
            "{\"durations\":[{\"seconds\":1234,\"nanos\":5678}]}"
        );

        let deserialized: MyMessage =
            serde_json::from_str(&result).expect("Unable to deserialize MyMessage");

        assert_eq!(deserialized.durations.len(), 1);
        assert_eq!(deserialized.durations[0].seconds, mm.durations[0].seconds);
        assert_eq!(deserialized.durations[0].nanos, mm.durations[0].nanos);
    }

    #[test]
    fn test_serde_fieldmask() {
        use crate::serde_fieldmask;
        use serde::{Deserialize, Serialize};
        use serde_json;

        #[derive(Serialize, Deserialize)]
        struct MyMessage {
            #[serde(with = "serde_fieldmask")]
            fm: Option<prost_types::FieldMask>,
        }

        let fm = prost_types::FieldMask {
            paths: vec![
                "path1".to_string(),
                "path2".to_string(),
                "path3".to_string(),
            ],
        };
        let mm = MyMessage { fm: Some(fm) };
        let result = serde_json::to_string(&mm).expect("Unable to serialize MyMessage");
        assert_eq!(
            result,
            "{\"fm\":{\"paths\":[\"path1\",\"path2\",\"path3\"]}}"
        );
    }
}
