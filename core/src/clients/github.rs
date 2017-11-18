#[cfg(test)]
mod tests {
    use github_rs::client::{Executor, Github};
    use serde_json::Value;

    #[test]
    fn client_github_works() {
        let client = Github::new("ad93b7981c41c2b423bf4bcb7672ceea00c84738").unwrap();
        let me = client.get()
                    .user()
                    .execute::<Value>();

        match me {
            Ok((headers, status, json)) => {
                println!("{}", headers);
                println!("{}", status);
                //assert_eq!(status, status::Ok);
                if let Some(json) = json{
                    println!("{}", json);
                    assert_eq!(json["login"], "ilerik");
                }
            },
            Err(e) => { 
                println!("{}", e);
                panic!("");
            }
        }
        
    }
}