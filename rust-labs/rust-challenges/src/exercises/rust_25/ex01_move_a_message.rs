/// A pending notification that can be inspected before it is sent.
///
/// Borrowing the text leaves the message available for sending, and copying the
/// numeric ID lets the caller keep tracking it afterward.
///
/// ```
/// use rust_challenges::exercises::rust_25::ex01_move_a_message::Message;
///
/// let message = Message::new(42, String::from("Deployment complete"));
/// let tracking_id = message.id();
/// assert_eq!(message.inspect(), "Deployment complete");
/// message.send();
/// assert_eq!(tracking_id, 42);
/// ```
///
/// A sent message cannot be used again:
///
/// ```compile_fail
/// use rust_challenges::exercises::rust_25::ex01_move_a_message::Message;
///
/// let message = Message::new(42, String::from("Deployment complete"));
/// message.send();
/// println!("{}", message.inspect()); // Ownership was transferred to send.
/// ```
///
/// Moving a message invalidates the original binding:
///
/// ```compile_fail
/// use rust_challenges::exercises::rust_25::ex01_move_a_message::Message;
///
/// let original = Message::new(7, String::from("Build passed"));
/// let transferred = original;
/// println!("{}", original.inspect()); // Ownership belongs to transferred.
/// transferred.send();
/// ```
#[derive(Debug)]
pub struct Message {
    id: u64,
    text: String,
}

impl Message {
    pub fn new(id: u64, text: String) -> Self {
        Self { id, text }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn inspect(&self) -> &str {
        &self.text
    }

    pub fn send(self) {
        println!("Sent #{}: {}", self.id, self.text);
    }
}

impl Clone for Message {
    fn clone(&self) -> Self {
        Message::new(self.id, self.text.clone())
    }
}

impl Drop for Message {
    fn drop(&mut self) {
        println!("Dropped message #{}", self.id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_and_send_preserves_copied_id() {
        let message = Message::new(42, String::from("Deployment complete"));

        let tracking_id = message.id();

        assert_eq!(tracking_id, 42);
        assert_eq!(message.inspect(), "Deployment complete");
        println!("Preview: {}", message.inspect());
        println!("Preview: {}", message.inspect());

        message.send();

        assert_eq!(tracking_id, 42);
        println!("Tracking #{tracking_id}");
    }

    #[test]
    fn inspect_borrows_the_owned_text() {
        let message = Message::new(42, String::from("Deployment complete"));
        let first_preview = message.inspect();
        let second_preview = message.inspect();

        assert_eq!(first_preview, "Deployment complete");
        assert_eq!(second_preview, first_preview);
        // A borrowed view must refer to the message's existing text storage.
        assert!(std::ptr::eq(first_preview, message.text.as_str()));
        assert!(std::ptr::eq(second_preview, first_preview));
    }

    #[test]
    fn moving_preserves_the_id_text_and_allocation() {
        let original = Message::new(7, String::from("Build passed"));
        let text_address = original.inspect().as_ptr();
        let transferred = original;

        assert_eq!(transferred.id(), 7);
        assert_eq!(transferred.inspect(), "Build passed");
        assert_eq!(transferred.inspect().as_ptr(), text_address);
        transferred.send();
    }

    #[test]
    fn clone_owns_independent_text_and_survives_sending_original() {
        let original = Message::new(9, String::from("Service restarted"));
        let mut duplicate = original.clone();

        assert_eq!(duplicate.id(), original.id());
        assert_eq!(duplicate.inspect(), original.inspect());

        // Tests in this module can access private fields to check independence.
        duplicate.text.push_str(" again");
        assert_eq!(original.inspect(), "Service restarted");
        assert_eq!(duplicate.inspect(), "Service restarted again");

        original.send();

        assert_eq!(duplicate.id(), 9);
        assert_eq!(duplicate.inspect(), "Service restarted again");
        duplicate.send();
    }

    #[test]
    fn accepts_boundary_ids_and_text_lengths() {
        for (id, text) in [(0, String::from("x")), (1_000_000_000, "x".repeat(1_000))] {
            let expected_length = text.len();
            let message = Message::new(id, text);

            assert_eq!(message.id(), id);
            assert_eq!(message.inspect().len(), expected_length);
            assert!(message.inspect().bytes().all(|byte| byte == b'x'));
        }
    }

    #[test]
    fn preserves_unicode_text_when_inspecting_and_cloning() {
        let text = "Déploiement terminé 🚀";
        let message = Message::new(42, String::from(text));
        let duplicate = message.clone();

        assert_eq!(message.inspect(), text);
        assert_eq!(duplicate.inspect(), text);
    }
}
