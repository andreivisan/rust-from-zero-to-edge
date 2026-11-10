use rust_challenges::exercises::rust_25::ex01_move_a_message::Message;
use std::process::Command;

// Run each scenario in a child test process to capture println! output using
// only std. This keeps stdout isolated when the parent tests run in parallel.
fn assert_stdout(test_name: &str, scenario: fn(), expected: &str) {
    const CHILD_TEST: &str = "RUST_25_MESSAGE_STDOUT_TEST";
    const BEGIN: &str = "MESSAGE_OUTPUT_BEGIN\n";
    const END: &str = "MESSAGE_OUTPUT_END\n";

    if std::env::var(CHILD_TEST).as_deref() == Ok(test_name) {
        print!("{BEGIN}");
        scenario();
        print!("{END}");
        return;
    }

    let output = Command::new(std::env::current_exe().expect("locate test executable"))
        .args(["--exact", test_name, "--nocapture", "--test-threads=1"])
        .env(CHILD_TEST, test_name)
        .output()
        .expect("run stdout scenario in a child process");

    assert!(
        output.status.success(),
        "scenario {test_name} failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let stdout = String::from_utf8(output.stdout).expect("scenario output is UTF-8");
    // Strip the test harness's own status messages, keeping scenario output exact.
    let (_, scenario_output) = stdout.split_once(BEGIN).expect("scenario start marker");
    let (actual, _) = scenario_output
        .split_once(END)
        .expect("scenario end marker");
    assert_eq!(actual, expected);
}

#[test]
fn preview_send_and_drop_match_example_1() {
    assert_stdout(
        "preview_send_and_drop_match_example_1",
        || {
            let message = Message::new(42, String::from("Deployment complete"));
            let tracking_id = message.id();

            println!("Preview: {}", message.inspect());
            println!("Preview: {}", message.inspect());
            message.send();
            println!("Tracking #{tracking_id}");
        },
        concat!(
            "Preview: Deployment complete\n",
            "Preview: Deployment complete\n",
            "Sent #42: Deployment complete\n",
            "Dropped message #42\n",
            "Tracking #42\n",
        ),
    );
}

#[test]
fn moving_drops_exactly_one_message_as_in_example_2() {
    assert_stdout(
        "moving_drops_exactly_one_message_as_in_example_2",
        || {
            let original = Message::new(7, String::from("Build passed"));
            let transferred = original;
            transferred.send();
        },
        "Sent #7: Build passed\nDropped message #7\n",
    );
}

#[test]
fn cloning_sends_and_drops_both_messages_as_in_example_3() {
    assert_stdout(
        "cloning_sends_and_drops_both_messages_as_in_example_3",
        || {
            let original = Message::new(9, String::from("Service restarted"));
            let duplicate = original.clone();

            original.send();
            duplicate.send();
        },
        concat!(
            "Sent #9: Service restarted\n",
            "Dropped message #9\n",
            "Sent #9: Service restarted\n",
            "Dropped message #9\n",
        ),
    );
}

#[test]
fn unsent_message_drops_at_scope_exit_as_in_example_4() {
    assert_stdout(
        "unsent_message_drops_at_scope_exit_as_in_example_4",
        || {
            {
                let message = Message::new(12, String::from("Deployment cancelled"));
                println!("Preview: {}", message.inspect());
            }

            println!("Scope finished");
        },
        concat!(
            "Preview: Deployment cancelled\n",
            "Dropped message #12\n",
            "Scope finished\n",
        ),
    );
}
