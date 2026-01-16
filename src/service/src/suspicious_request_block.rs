static START_WITH_TARGETS: [&str; 13] = [
    "/201",
    "/202",
    "/../",
    "/_",
    "/.aws/",
    "/.git/",
    "/.kube/",
    "/.ssh/",
    "/.well-knownold/",
    "/blog",
    "/docker-compose",
    "/etc/",
    "/http",
];
static ENDS_WITH_TARGETS: [&str; 9] = [
    ".env", ".json", ".yaml", ".yml", ".php", ".sql", ".tar.gz", ".xml", ".zip",
];
static CONTAINS_TARGETS: [&str; 1] = ["/wp-"];

/// Scans the given url path for common scan targets from bots and attackers.
pub fn is_scan_target(request_path: &str) -> bool {
    // chrome can make this request to the backend when dev tools are open
    if request_path.ends_with("/com.chrome.devtools.json") {
        return false;
    }

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
