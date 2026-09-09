use std::collections::HashSet;

use arma_rs::Group;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use std::sync::Mutex;
use std::time::{Duration, Instant};

// xml clockwork:
#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
struct Member {
    #[serde(rename = "@id")]
    id: String,
    // #[serde(rename = "@nick")]
    // nick: String,
}
#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename = "squad")]
struct Squad {
    #[serde(rename = "member")]
    members: Vec<Member>,
}
struct MemberCache {
    members: HashSet<String>,
    last_updated: Instant,
}
static MEMBER_CACHE: LazyLock<Mutex<MemberCache>> = LazyLock::new(|| {
    Mutex::new(MemberCache {
        members: HashSet::new(),
        last_updated: Instant::now() - Duration::from_secs(999999), // force update on first call
    })
});
const CACHE_EXPIRATION: Duration = Duration::from_secs(2 * 60 * 60); // 2 hours

pub fn group() -> Group {
    Group::new().command("is_member", is_member)
}

/// Check if a given ID is a member of the squad.xml
fn is_member(id: String) -> Result<bool, bool> {
    let mut cache = MEMBER_CACHE.lock().unwrap();
    if cache.last_updated.elapsed() >= CACHE_EXPIRATION {
        println!("Cache expired, updating squad data...");
        match update_squad_data(&mut cache.members) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to update squad data: {}", e);
            }
        }
        cache.last_updated = Instant::now();
    }
    Ok(cache.members.contains(&id))
}
/// Get the latest squad data from the server and update the cache.
fn update_squad_data(members: &mut HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("potato")
        .timeout(Duration::from_secs(1))
        .build()?;
    let resp = client.get("https://squad.bourbonwarfare.com").send()?;
    let text = resp.text()?;
    let squad: Squad = quick_xml::de::from_str(&text)?;

    for member in squad.members {
        members.insert(member.id);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use arma_rs::Extension;
    use std::vec;
    #[test]
    fn test1() {
        let extension = Extension::build()
            .command("is_member", is_member)
            .finish()
            .testing();
        let (output, _) = extension.call("is_member", Some(vec!["123".to_owned()]));
        assert_eq!(output, "false");
        let (output, _) = extension.call("is_member", Some(vec!["76561197970715242".to_owned()]));
        assert_eq!(output, "true");
    }
}
