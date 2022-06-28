pub struct GetAuthUrlOptions {
    force_prompt: Option<bool>,
    state: Option<String>,
    make_bypass: Option<String>,
    single_select: Option<bool>,
    single_select_by_vin: Option<String>,
}

impl GetAuthUrlOptions {
    pub fn new() -> GetAuthUrlOptions {
        GetAuthUrlOptions {
            force_prompt: None,
            state: None,
            make_bypass: None,
            single_select_by_vin: None,
            single_select: None,
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

    pub fn build_query_string(self) -> String {
        let mut query_string = String::from("");

        if let Some(enabled) = self.force_prompt {
            if enabled == true {
                query_string.push_str("&approval_prompt=");
                query_string.push_str("force");
            }
        };

        if let Some(state) = self.state {
            query_string.push_str("&state=");
            query_string.push_str(state.as_str());
        }

        if let Some(make) = self.make_bypass {
            query_string.push_str("&make=");
            query_string.push_str(make.as_str());
        }

        match self.single_select_by_vin {
            Some(vin) => {
                query_string.push_str("&single_select_vin=");
                query_string.push_str(vin.as_str());
                query_string.push_str("&single_select=true");
            }
            None => {
                if let Some(enabled) = self.single_select {
                    query_string.push_str("&single_select=");
                    if enabled == true {
                        query_string.push_str("true");
                    } else {
                        query_string.push_str("false ");
                    }
                }
            }
        }

        query_string
    }
}

#[test]
fn get_auth_url_options_query_build() {
    let query = GetAuthUrlOptions::new()
        .set_make_bypass("mercedes".to_string())
        .set_state("no-michael-no-no-michael".to_string())
        .set_single_select_by_vin("THATISSONOTRIGHT".to_string())
        .set_force_prompt(true)
        .build_query_string();

    let expecting = "&approval_prompt=force\
        &state=no-michael-no-no-michael\
        &make=mercedes\
        &single_select_vin=THATISSONOTRIGHT\
        &single_select=true";

    assert_eq!(query, String::from(expecting));
}
