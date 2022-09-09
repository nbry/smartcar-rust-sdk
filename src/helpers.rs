use std::{collections::HashMap, env};

pub(crate) fn get_api_url() -> String {
    match env::var("SMARTCAR_API_ORIGIN") {
        Ok(api_url) => api_url,
        Err(_) => String::from("https://api.smartcar.com"),
    }
}

pub(crate) fn get_oauth_url() -> String {
    match env::var("SMARTCAR_AUTH_ORIGIN") {
        Ok(api_url) => api_url,
        Err(_) => String::from("https://auth.smartcar.com/oauth/token"),
    }
}

pub(crate) fn get_connect_url() -> String {
    match env::var("SMARTCAR_CONNECT_URL") {
        Ok(api_url) => api_url,
        Err(_) => String::from("https://connect.smartcar.com"),
    }
}

/// Get the request query value for flags
///
/// Note: Does not include the &flag= query key
pub(crate) fn format_flag_query(flags: &HashMap<String, String>) -> String {
    let mut query = String::from("");
    if flags.keys().len() == 0 {
        return query;
    };

    flags.keys().into_iter().for_each(|flag| {
        let value = flags.get(flag);

        if let Some(v) = value {
            let flag_formatted = format!("{}:{} ", flag, v);
            query.push_str(&flag_formatted);
        };
    });

    query.trim_end().to_string()
}

#[test]
fn formatting_flag_query() {
    let mut flags = HashMap::new();
    flags.insert(String::from("black"), String::from("flag"));
    flags.insert(String::from("good"), String::from("band"));

    let flag_query = format_flag_query(&flags);

    // Order not preserved
    assert!(flag_query.contains("black:flag"));
    assert!(flag_query.contains(" "));
    assert!(flag_query.contains("good:band"));
}
