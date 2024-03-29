use thiserror::Error;

#[derive(Error, Debug)]
pub enum CfgError {
    #[error("failed to load value for key {0}: {1}")]
    LoadFailed(String, String),
}

#[derive(strum_macros::Display)]
pub enum Cfg {
    DatabaseUrl,
}

/// Convert CamelCase to snake_case
fn to_snake_case(in_str: &str) -> String {
    let (mut str_vec, last_str): (Vec<String>, String) =
        in_str.chars().fold((vec![], String::new()), |acc, el| {
            let (mut str_vec, mut cur) = acc;
            if el.is_ascii_uppercase() && !cur.is_empty() {
                str_vec.push(cur);
                let cur: String = el.into();
                (str_vec, cur)
            } else {
                cur.push(el.to_ascii_uppercase());
                (str_vec, cur)
            }
        });
    str_vec.push(last_str);
    let num_strs = str_vec.len();
    let out_str: String = str_vec
        .into_iter()
        .enumerate()
        .fold(String::new(), |acc, e| {
            let (i, s) = e;
            if i == num_strs - 1 {
                format!("{acc}{s}")
            } else {
                format!("{acc}{s}_")
            }
        });

    out_str
}

impl Cfg {
    pub fn load(&self) -> Result<String, CfgError> {
        // Load env-vars to match attributes of Cfg enum
        // We're using strum to convert the variant names to string, then converting to SNAKE_CASE
        // to get the env-var name.

        let key: String = to_snake_case(&self.to_string());
        dotenv::var(&key).map_err(|e| CfgError::LoadFailed(key, e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_snake_case_convertion() {
        assert_eq!(to_snake_case("HelloWorld"), "HELLO_WORLD".to_string());
        assert_eq!(
            to_snake_case("WowThisIsWeird"),
            "WOW_THIS_IS_WEIRD".to_string()
        );
        assert_eq!(to_snake_case("OMG"), "O_M_G".to_string());
    }
}
