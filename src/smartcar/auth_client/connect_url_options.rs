pub struct ConnectUrlOptions {
    force_prompt: Option<bool>,
    state: Option<String>,
    make_bypass: Option<String>,
    single_select: Option<SingleSelect>,
}

pub struct SingleSelect {
    vin: Option<String>,
    enabled: Option<bool>,
}
