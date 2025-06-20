const START_WITH_TARGETS: [&str; 14] = [
    "/2018",
    "/2019",
    "/2020",
    "/../",
    "/.aws/",
    "/.env",
    "/.git/",
    "/.kube/",
    "/.ssh/",
    "/.well-knownold/",
    "/blog",
    "/docker-compose",
    "/etc/",
    "/http",
];

const ENDS_WITH_TARGETS: [&str; 8] = [
    ".json", ".yaml", ".yml", ".php", ".sql", ".xml", ".tar.gz", ".zip",
];

const CONTAINS_TARGETS: [&str; 1] = ["/wp-"];
/// Scans the given url path for common scan targets from bots and attackers.
pub fn is_scan_target(request_path: &str) -> bool {
    for target in START_WITH_TARGETS {
        if request_path.starts_with(target) {
            return true;
        }
    }

    for target in ENDS_WITH_TARGETS {
        if request_path.ends_with(target) {
            return true;
        }
    }

    for target in CONTAINS_TARGETS {
        if request_path.contains(target) {
            return true;
        }
    }

    false
}
