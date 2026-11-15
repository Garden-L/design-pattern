use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_notification-cli"))
        .args(args)
        .output()
        .expect("run notification CLI")
}

#[test]
fn email_command_builds_report_and_applies_discount() {
    let result = run(&[
        "--channel",
        "email",
        "--title",
        "Release",
        "--message",
        "Ready",
        "--pricing",
        "discount",
    ]);
    assert!(result.status.success(), "{:?}", result);
    assert_eq!(
        String::from_utf8(result.stdout).unwrap(),
        "EMAIL: Release\nReady\nPrice: 90\n"
    );
    assert!(result.stderr.is_empty());
}

#[test]
fn sms_command_uses_regular_price_by_default_and_preserves_unicode() {
    let result = run(&["--channel", "sms", "--title", "알림", "--message", "완료"]);
    assert!(result.status.success());
    assert_eq!(
        String::from_utf8(result.stdout).unwrap(),
        "SMS: 알림\n완료\nPrice: 100\n"
    );
}

#[test]
fn invalid_commands_fail_without_delivering() {
    let fixtures: &[(&[&str], &str)] = &[
        (&[], "missing --channel"),
        (&["--channel", "push"], "unsupported channel"),
        (&["--channel", "email", "--title", "T"], "missing --message"),
        (
            &["--channel", "email", "--title", " ", "--message", "B"],
            "MissingTitle",
        ),
        (
            &["--channel", "email", "--title", "T", "--message", " "],
            "MissingBody",
        ),
        (
            &[
                "--channel",
                "email",
                "--title",
                "T",
                "--message",
                "B",
                "--pricing",
                "bogus",
            ],
            "unsupported pricing",
        ),
        (&["--unknown", "x"], "unknown option"),
        (&["--channel"], "missing value"),
        (
            &["--channel", "email", "--channel", "sms"],
            "duplicate option",
        ),
    ];
    for (args, message) in fixtures {
        let result = run(args);
        assert_eq!(result.status.code(), Some(2), "{args:?}: {result:?}");
        assert!(result.stdout.is_empty(), "{args:?}");
        assert!(
            String::from_utf8(result.stderr).unwrap().contains(message),
            "{args:?}"
        );
    }
}

#[test]
fn help_succeeds_without_required_options() {
    let result = run(&["--help"]);
    assert!(result.status.success());
    assert!(String::from_utf8(result.stdout)
        .unwrap()
        .contains("--channel email|sms"));
    assert!(result.stderr.is_empty());
}
