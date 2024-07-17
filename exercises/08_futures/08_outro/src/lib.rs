// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.

pub struct Ticket {
    id: u64,
    title: String,
    description: String,
    status: String,
}

pub struct TicketManager {
    tickets: Vec<Ticket>,
}

impl TicketManager {
    pub fn new() -> Self {
        Self { tickets: Vec::new() }
    }

    pub async fn create_ticket(&mut self, title: String, description: String) -> u64 {
        let id = self.tickets.len() as u64;
        self.tickets.push(Ticket {
            id,
            title,
            description,
            status: "open".to_string(),
        });
        id
    }

    pub async fn get_ticket(&self, id: u64) -> Option<&Ticket> {
        self.tickets.iter().find(|ticket| ticket.id == id)
    }

    pub async fn patch_ticket(&mut self, id: u64, title: Option<String>, description: Option<String>, status: Option<String>) -> bool {
        if let Some(ticket) = self.tickets.iter_mut().find(|ticket| ticket.id == id) {
            if let Some(title) = title {
                ticket.title = title;
            }
            if let Some(description) = description {
                ticket.description = description;
            }
            if let Some(status) = status {
                ticket.status = status;
            }
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_ticket() {
        let mut manager = TicketManager::new();
        let id = manager.create_ticket("title".to_string(), "description".to_string()).await;
        assert_eq!(id, 0);
    }

    #[tokio::test]
    async fn get_ticket() {
        let mut manager = TicketManager::new();
        let id = manager.create_ticket("title".to_string(), "description".to_string()).await;
        let ticket = manager.get_ticket(id).await.unwrap();
        assert_eq!(ticket.id, id);
        assert_eq!(ticket.title, "title");
        assert_eq!(ticket.description, "description");
        assert_eq!(ticket.status, "open");
    }

    #[tokio::test]
    async fn patch_ticket() {
        let mut manager = TicketManager::new();
        let id = manager.create_ticket("title".to_string(), "description".to_string()).await;
        let result = manager.patch_ticket(id, Some("new title".to_string()), Some("new description".to_string()), Some("closed".to_string())).await;
        assert!(result);
        let ticket = manager.get_ticket(id).await.unwrap();
        assert_eq!(ticket.id, id);
        assert_eq!(ticket.title, "new title");
        assert_eq!(ticket.description, "new description");
        assert_eq!(ticket.status, "closed");
    }
}