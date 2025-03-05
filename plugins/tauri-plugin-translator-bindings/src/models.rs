use gtranslate::TranslateOptions;
use serde::{de::Visitor, ser::SerializeMap, Deserialize, Serialize};
use std::time::Duration;
use serde_with::{serde_as, DurationSeconds};
use thiserror::Error;

#[serde_as]
#[derive(Serialize, Debug, Deserialize)]
pub struct AppConfig {
  #[serde_as(as = "DurationSeconds<String>")]
  pub timeout: Duration,
  pub proxy: Option<String>,
  #[serde(rename = "useProxy")]
  pub use_proxy: Option<bool>,
  pub autostart: Option<bool>
}

#[derive(Error, Debug, Serialize)]
pub enum AppError {
  #[error("error: {wrapped}")]
  WrapError{wrapped: String}
}

pub struct TranslationOpts<'a> {
  pub opts: gtranslate::TranslateOptions<'a>
}

impl std::fmt::Debug for TranslationOpts<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(
          f, 
          "client: {}\nsource_lang: {}\ntarget_lang: {}\ndst_target: {}\nquery: {}\n", 
          self.opts.client,
          self.opts.source_lang,
          self.opts.target_lang,
          self.opts.dst_target,
          self.opts.query
      )
    }
}

struct OptsVisitor;

impl<'de> Visitor<'de> for OptsVisitor {
    type Value = TranslationOpts<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
      formatter.write_str("a str")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
        
        let mut translation_opts = TranslationOpts::<'de>{
          opts: TranslateOptions::new()
        };

        while let Some((key, value)) = map.next_entry::<&'de str, &'de str>()? {
            match key {
                "client" => translation_opts.opts.client = if !value.is_empty() { value } else { "gtx" },
                "source_lang" => translation_opts.opts.source_lang = value,
                "target_lang" => translation_opts.opts.target_lang = value,
                "dst_target" => translation_opts.opts.dst_target = if !value.is_empty() { value } else { "t" },
                "query" => translation_opts.opts.query = value,
                _ => {
                  return Err(serde::de::Error::custom("translation options misses fields"))
                }
            }
        }
        
        Ok(translation_opts)
    }
}

impl<'de> Deserialize<'de> for TranslationOpts<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Ok(deserializer.deserialize_map(OptsVisitor)?)
    }
}

impl<'se> Serialize for TranslationOpts<'se> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut map = serializer.serialize_map(None)?;
        
        map.serialize_entry("client", "gtx")?;
        map.serialize_entry("source_lang", self.opts.source_lang)?;
        map.serialize_entry("target_lang", self.opts.target_lang)?;
        map.serialize_entry("dst_target:", self.opts.dst_target)?;
        map.serialize_entry("query:", self.opts.query)?;

        map.end()
    }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_deserializer() {
    use crate::TranslationOpts;

    let deserialized: TranslationOpts<'_> = serde_json::from_str("{\"client\": \"gtx\", \"source_lang\": \"nl\", \"target_lang\": \"tr\", \"dst_target\": \"tr\",\"query\": \"ik ga hardlopen\"}").unwrap();
    println!("deserialized: {:?}", deserialized);

    println!("serialized: {:?}", serde_json::to_string(&deserialized));
  }
}