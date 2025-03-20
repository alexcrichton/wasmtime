use std::process::Command;

fn main() {
    let output = Command::new("git")
        .arg("for-each-ref")
        .arg("refs/remotes/origin")
        .arg("--format")
        .arg("%(refname)")
        .output()
        .unwrap();
    assert!(output.status.success());
    let mut releases = std::str::from_utf8(&output.stdout)
        .unwrap()
        .lines()
        .filter_map(|l| l.strip_prefix("refs/remotes/origin/release-"))
        .filter_map(|l| {
            let mut parts = l.split('.');
            let major = parts.next()?.parse::<u32>().ok()?;
            let minor = parts.next()?.parse::<u32>().ok()?;
            let patch = parts.next()?.parse::<u32>().ok()?;
            Some((major, minor, patch))
        })
        .collect::<Vec<_>>();
    releases.sort();

    let mut to_trigger: Vec<(u32, u32, u32)> = Vec::new();
    let mut iter = releases.iter().rev();

    to_trigger.extend(iter.by_ref().take(3));

    let mut lts_channels = 2;
    if to_trigger.iter().any(|(major, _, _)| *major % 12 == 0) {
        lts_channels -= 1;
    }

    to_trigger.extend(
        iter.filter(|(major, _, _)| *major % 12 == 0 && *major > 20)
            .take(lts_channels),
    );
    println!("{to_trigger:?}");

    for (major, minor, patch) in to_trigger {
        dbg!(major, minor, patch);
        let status = Command::new("gh")
            .arg("workflow")
            .arg("run")
            .arg("main.yml")
            .arg("--ref")
            .arg(format!("release-{major}.{minor}.{patch}"))
            .status()
            .unwrap();
        assert!(status.success());
    }
}
