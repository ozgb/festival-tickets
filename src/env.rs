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
fn to_snake_case(in_str: String) -> String {
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
        let key: String = to_snake_case(self.to_string());
        dotenv::var(&key).map_err(|e| CfgError::LoadFailed(key, e.to_string()))
    }
}
