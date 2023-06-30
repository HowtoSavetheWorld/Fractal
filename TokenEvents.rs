use std::collections::VecDeque;

// Events module
mod events {
    use std::time::SystemTime;

    pub enum EventType {
        TokenTransfer {
            sender: String,
            receiver: String,
            amount: u64,
        },
        Approval {
            owner: String,
            spender: String,
            amount: u64,
        },
        // Add more event types as needed
    }

    pub struct Event {
        event_type: EventType,
        payload: EventPayload,
        timestamp: SystemTime,
        id: u64,
    }

    pub enum EventPayload {
        TokenTransferPayload(TokenTransferPayload),
        ApprovalPayload(ApprovalPayload),
        // Add more payload types for different event types
    }

    pub struct TokenTransferPayload {
        sender: String,
        receiver: String,
        amount: u64,
    }

    pub struct ApprovalPayload {
        owner: String,
        spender: String,
        amount: u64,
    }

    pub trait EventHandler {
        fn handle_event(&self, event: Event);
        fn handle_token_transfer(&self, payload: TokenTransferPayload);
        fn handle_approval(&self, payload: ApprovalPayload);
        // Implement methods for handling other event types
    }

    pub struct DefaultEventHandler {
        event_history: VecDeque<Event>,
    }

    impl DefaultEventHandler {
        fn new() -> Self {
            Self {
                event_history: VecDeque::new(),
            }
        }

        fn add_to_event_history(&mut self, event: Event) {
            self.event_history.push_back(event);
        }
    }

    impl EventHandler for DefaultEventHandler {
        fn handle_event(&self, event: Event) {
            // Handle event logic
            println!("Handling event: {:?}", event);
        }

        fn handle_token_transfer(&self, payload: TokenTransferPayload) {
            // Handle token transfer event logic
            println!("Handling token transfer: {:?}", payload);
        }

        fn handle_approval(&self, payload: ApprovalPayload) {
            // Handle approval event logic
            println!("Handling approval: {:?}", payload);
        }
    }
}

struct UserManager<E: events::EventHandler> {
    event_handler: E,
}

impl<E: events::EventHandler> UserManager<E> {
    fn new(event_handler: E) -> Self {
        Self {
            event_handler,
        }
    }

    fn perform_event(&self, event: events::Event) {
        self.event_handler.handle_event(event.clone());
        self.event_handler.add_to_event_history(event);
    }

    fn perform_token_transfer(&self, payload: events::TokenTransferPayload) {
        let event = events::Event {
            event_type: events::EventType::TokenTransfer {
                sender: payload.sender.clone(),
                receiver: payload.receiver.clone(),
                amount: payload.amount,
            },
            payload: events::EventPayload::TokenTransferPayload(payload),
            timestamp: SystemTime::now(),
            id: 0, // Assign a unique ID for the event
        };

        self.event_handler.handle_token_transfer(payload);
        self.event_handler.add_to_event_history(event);
    }

    fn perform_approval(&self, payload: events::ApprovalPayload) {
        let event = events::Event {
            event_type: events::EventType::Approval {
                owner: payload.owner.clone(),
                spender: payload.spender.clone(),
                amount: payload.amount,
            },
            payload: events::EventPayload::ApprovalPayload(payload),
            timestamp: SystemTime::now(),
            id: 0, // Assign a unique ID for the event
        };

        self.event_handler.handle_approval(payload);
        self.event_handler.add_to_event_history(event);
    }
}

fn main() {
    let mut event_handler = events::DefaultEventHandler::new();
    let user_manager = UserManager::new(&mut event_handler);

    let token_transfer_payload = events::TokenTransferPayload {
        sender: "Alice".to_string(),
        receiver: "Bob".to_string(),
        amount: 100,
    };

    let approval_payload = events::ApprovalPayload {
        owner: "Alice".to_string(),
        spender: "Charlie".to_string(),
        amount: 50,
    };

    user_manager.perform_token_transfer(token_transfer_payload);
    user_manager.perform_approval(approval_payload);

    // Accessing event history
    for event in event_handler.event_history.iter() {
        println!("Event: {:?}", event);
    }
}