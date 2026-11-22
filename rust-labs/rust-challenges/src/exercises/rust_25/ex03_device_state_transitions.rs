#[derive(Debug, PartialEq, Eq)]
pub enum DeviceState {
    Disconnected,
    Connecting {
        attempt: u8,
    },
    Connected {
        device_id: String,
        latest_reading: Option<i32>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    Connect,
    ConnectionEstablished { device_id: String },
    Reading(i32),
    Timeout,
    Disconnect,
}

pub fn transition(state: DeviceState, event: Event) -> DeviceState {
    match (state, event) {
        (DeviceState::Disconnected, Event::Connect) => DeviceState::Connecting { attempt: 1 },
        (DeviceState::Connecting { attempt }, Event::Timeout) if attempt < 3 => {
            DeviceState::Connecting {
                attempt: attempt + 1,
            }
        }
        (DeviceState::Connecting { attempt: 3 }, Event::Timeout) => DeviceState::Disconnected,
        (DeviceState::Connecting { .. }, Event::ConnectionEstablished { device_id }) => {
            DeviceState::Connected {
                device_id,
                latest_reading: None,
            }
        }
        (DeviceState::Connected { device_id, .. }, Event::Reading(value)) => {
            DeviceState::Connected {
                device_id,
                latest_reading: Some(value),
            }
        }
        (_, Event::Disconnect) => DeviceState::Disconnected,
        (unchanged_state, _) => unchanged_state,
    }
}

pub fn latest_reading(state: &DeviceState) -> Option<i32> {
    match state {
        DeviceState::Connected { latest_reading, .. } if latest_reading.is_none() => None,
        DeviceState::Connected { latest_reading, .. } => *latest_reading,
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{DeviceState, Event, latest_reading, transition};

    #[test]
    fn connect_starts_the_first_attempt() {
        assert_eq!(
            transition(DeviceState::Disconnected, Event::Connect),
            DeviceState::Connecting { attempt: 1 },
        );
    }

    #[test]
    fn disconnected_ignores_unexpected_events() {
        for event in [
            Event::ConnectionEstablished {
                device_id: String::from("sensor-7"),
            },
            Event::Reading(21),
            Event::Timeout,
        ] {
            assert_eq!(
                transition(DeviceState::Disconnected, event),
                DeviceState::Disconnected
            );
        }
    }

    #[test]
    fn timeout_advances_attempts_one_and_two() {
        for attempt in [1, 2] {
            assert_eq!(
                transition(DeviceState::Connecting { attempt }, Event::Timeout),
                DeviceState::Connecting {
                    attempt: attempt + 1
                },
            );
        }
    }

    #[test]
    fn timeout_on_the_third_attempt_disconnects() {
        assert_eq!(
            transition(DeviceState::Connecting { attempt: 3 }, Event::Timeout),
            DeviceState::Disconnected,
        );
    }

    #[test]
    fn connection_establishment_moves_the_id_and_starts_without_a_reading() {
        for attempt in [1, 2, 3] {
            let device_id = String::from("sensor-7");
            let allocation = device_id.as_ptr();
            let next = transition(
                DeviceState::Connecting { attempt },
                Event::ConnectionEstablished { device_id },
            );

            assert_eq!(
                next,
                DeviceState::Connected {
                    device_id: String::from("sensor-7"),
                    latest_reading: None,
                },
            );
            if let DeviceState::Connected { device_id, .. } = &next {
                assert_eq!(device_id.as_ptr(), allocation);
            }
        }
    }

    #[test]
    fn a_repeated_connect_request_is_ignored_while_connecting() {
        for attempt in [1, 2, 3] {
            assert_eq!(
                transition(DeviceState::Connecting { attempt }, Event::Connect),
                DeviceState::Connecting { attempt },
                "a repeated connect request must preserve attempt {attempt}",
            );
        }
    }

    #[test]
    fn readings_are_ignored_while_connecting() {
        for attempt in [1, 2, 3] {
            assert_eq!(
                transition(DeviceState::Connecting { attempt }, Event::Reading(42)),
                DeviceState::Connecting { attempt },
            );
        }
    }

    #[test]
    fn disconnect_works_from_every_state() {
        for state in [
            DeviceState::Disconnected,
            DeviceState::Connecting { attempt: 2 },
            DeviceState::Connected {
                device_id: String::from("sensor-7"),
                latest_reading: Some(18),
            },
        ] {
            assert_eq!(
                transition(state, Event::Disconnect),
                DeviceState::Disconnected
            );
        }
    }

    #[test]
    fn readings_replace_the_previous_value_and_preserve_the_owned_id() {
        for previous in [None, Some(21)] {
            for value in [-10_000, 0, 10_000] {
                let device_id = String::from("sensor-7");
                let allocation = device_id.as_ptr();
                let next = transition(
                    DeviceState::Connected {
                        device_id,
                        latest_reading: previous,
                    },
                    Event::Reading(value),
                );

                assert_eq!(
                    next,
                    DeviceState::Connected {
                        device_id: String::from("sensor-7"),
                        latest_reading: Some(value),
                    },
                );
                if let DeviceState::Connected { device_id, .. } = &next {
                    assert_eq!(device_id.as_ptr(), allocation);
                }
            }
        }
    }

    #[test]
    fn connected_ignores_unexpected_events_and_preserves_its_reading() {
        for event in [
            Event::Connect,
            Event::ConnectionEstablished {
                device_id: String::from("sensor-99"),
            },
            Event::Timeout,
        ] {
            let state = DeviceState::Connected {
                device_id: String::from("sensor-7"),
                latest_reading: Some(-5),
            };
            assert_eq!(
                transition(state, event),
                DeviceState::Connected {
                    device_id: String::from("sensor-7"),
                    latest_reading: Some(-5),
                },
            );
        }
    }

    #[test]
    fn latest_reading_distinguishes_absence_from_zero_and_preserves_the_state() {
        for (state, expected) in [
            (DeviceState::Disconnected, None),
            (DeviceState::Connecting { attempt: 1 }, None),
            (
                DeviceState::Connected {
                    device_id: String::from("sensor-7"),
                    latest_reading: None,
                },
                None,
            ),
            (
                DeviceState::Connected {
                    device_id: String::from("sensor-7"),
                    latest_reading: Some(0),
                },
                Some(0),
            ),
            (
                DeviceState::Connected {
                    device_id: String::from("sensor-7"),
                    latest_reading: Some(-5),
                },
                Some(-5),
            ),
            (
                DeviceState::Connected {
                    device_id: String::from("sensor-7"),
                    latest_reading: Some(21),
                },
                Some(21),
            ),
        ] {
            assert_eq!(latest_reading(&state), expected);
            assert_eq!(latest_reading(&state), expected);
            assert_eq!(
                transition(state, Event::Disconnect),
                DeviceState::Disconnected
            );
        }
    }

    #[test]
    fn reconnecting_starts_with_a_new_id_and_no_previous_reading() {
        let state = DeviceState::Connected {
            device_id: String::from("sensor-7"),
            latest_reading: Some(18),
        };
        let state = transition(state, Event::Disconnect);
        let state = transition(state, Event::Connect);
        let state = transition(
            state,
            Event::ConnectionEstablished {
                device_id: String::from("sensor-99"),
            },
        );

        assert_eq!(
            state,
            DeviceState::Connected {
                device_id: String::from("sensor-99"),
                latest_reading: None,
            },
        );
        assert_eq!(latest_reading(&state), None);
    }
}
