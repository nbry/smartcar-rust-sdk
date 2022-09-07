use std::collections::HashMap;

use crate::{helpers, request::MultiQuery};

pub struct AuthUrlOptionsBuilder {
    force_prompt: Option<bool>,
    state: Option<String>,
    make_bypass: Option<String>,
    single_select: Option<bool>,
    single_select_by_vin: Option<String>,
    flags: Option<HashMap<String, String>>,
}

impl AuthUrlOptionsBuilder {
    pub fn new() -> AuthUrlOptionsBuilder {
        AuthUrlOptionsBuilder {
            force_prompt: None,
            state: None,
            make_bypass: None,
            single_select_by_vin: None,
            single_select: None,
            flags: None,
        }
    }

    pub fn set_force_prompt(mut self, enabled: bool) -> Self {
        self.force_prompt = Some(enabled);
        self
    }

    pub fn set_state(mut self, state: String) -> Self {
        self.state = Some(state);
        self
    }

    pub fn set_make_bypass(mut self, make: String) -> Self {
        self.make_bypass = Some(make);
        self
    }

    pub fn set_single_select(mut self, enabled: bool) -> Self {
        self.single_select = Some(enabled);
        self
    }

    pub fn set_single_select_by_vin(mut self, vin: String) -> Self {
        self.single_select_by_vin = Some(vin);
        self
    }

    pub fn set_flags(mut self, flags: &HashMap<String, String>) -> Self {
        self.flags = Some(flags.to_owned());
        self
    }
}

impl MultiQuery for AuthUrlOptionsBuilder {
    fn vectorize(&self) -> Vec<(String, String)> {
        let mut query_string = Vec::new();

        if let Some(enabled) = self.force_prompt {
            if enabled == true {
                query_string.push(("approval_prompt".to_string(), "force".to_string()));
            }
        };

        if let Some(state) = &self.state {
            query_string.push(("state".to_string(), state.to_owned()));
        }

        if let Some(make) = &self.make_bypass {
            query_string.push(("make".to_string(), make.to_owned()));
        }

        if let Some(flags) = &self.flags {
            let flag_query = helpers::format_flag_query(flags);
            query_string.push(("flag".to_string(), flag_query.to_owned()));
        }

        match &self.single_select_by_vin {
            Some(vin) => {
                query_string.push(("single_select_vin".to_string(), vin.to_owned()));
                query_string.push(("single_select".to_string(), "true".to_string()));
            }
            None => {
                if let Some(enabled) = &self.single_select {
                    query_string.push(("single_select".to_string(), enabled.to_string()));
                }
            }
        }
        query_string
    }
}

#[test]
fn get_auth_url_options_query_build() {
    let options = AuthUrlOptionsBuilder::new()
        .set_make_bypass("mercedes".to_string())
        .set_state("no-michael-no-no-michael".to_string())
        .set_single_select_by_vin("THATISSONOTRIGHT".to_string())
        .set_force_prompt(true);

    let query = options.vectorize();

    let expecting = vec![
        ("approval_prompt".to_string(), "force".to_string()),
        ("state".to_string(), "no-michael-no-no-michael".to_string()),
        ("make".to_string(), "mercedes".to_string()),
        (
            "single_select_vin".to_string(),
            "THATISSONOTRIGHT".to_string(),
        ),
        ("single_select".to_string(), "true".to_string()),
    ];

    // O(n^2)... "shrugs"
    assert!(query.iter().all(|q| expecting.contains(q)));
}
