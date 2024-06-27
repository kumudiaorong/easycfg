use tracing::debug;

use super::dis::Distri;

const OS_RELEASE_PATHS: [&str; 2] = ["/etc/os-release", "/usr/lib/os-release"];
const OS_RELEASE_ID_FLAGS: [(Distri, &str); 2] =
    [(Distri::Arch, "arch"), (Distri::OpenSUSE, "opensuse")];

pub fn get_release() -> Distri {
    let reg = regex::Regex::new(
        r#"^(?P<key>[a-zA-Z0-9_]+)=(?:["']?[[:space:]]*)(?P<value>.*?)(?:[[:space:]]*["']?)$"#,
    )
    .unwrap();
    for path in OS_RELEASE_PATHS.iter() {
        let file = std::fs::read_to_string(path);
        if file.is_err() {
            continue;
        }
        let file = file.unwrap();
        let mut distri = Distri::Unknown;
        for line in file.lines() {
            let caps = reg.captures(line.trim());
            if caps.is_none() {
                continue;
            }
            let caps = caps.unwrap();
            let key = caps.name("key").unwrap().as_str();
            let value = caps.name("value").unwrap().as_str();
            if key == "ID" {
                for (d, id) in OS_RELEASE_ID_FLAGS.iter() {
                    if value.contains(id) {
                        distri = *d;
                        break;
                    }
                }
            }
            debug!("{}: {}", key, value);
        }
        return distri;
    }
    Distri::Unknown
}
