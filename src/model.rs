//! Simplistic Model layer
//! (with mock-store layer)

use crate::{Error, Result, ctx::Ctx};
use serde::{Deserialize, Serialize};
use std::{sync::{Arc, Mutex}};

// region: --- Ticket Types
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub cid: u64, // creator user_id
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}
// endregion: --- Ticket Types

// region: --- Model Controller
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// Constructor
impl ModelController {
    pub async fn new() -> Self {
        Self {
            tickets_store: Arc::default(),
        }
    }
}

// CRUD for ModelController
// crud functions for the ModelController
impl ModelController {
    pub async fn create_ticket(&self, ctx: Ctx, ticket: TicketForCreate) -> Result<Ticket> {
        let mut tickets_store = self.tickets_store.lock().unwrap();
        let id = tickets_store.len() as u64;
        let ticket = Ticket {
            id,
            cid: ctx.user_id(),
            title: ticket.title,
        };
        tickets_store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        let tickets_store = self.tickets_store.lock().unwrap();
        let tickets = tickets_store
            .iter()
            .filter_map(|ticket| ticket.clone())
            .collect();
        Ok(tickets)
    }

    pub async fn get_ticket(&self, _ctx: Ctx, id: u64) -> Result<Ticket> {
        let tickets_store = self.tickets_store.lock().unwrap();
        let ticket = tickets_store
            .get(id as usize)
            .ok_or(Error::TicketGetNotFound{id})?
            .clone()
            .ok_or(Error::TicketGetNotFound{id})?;
        Ok(ticket)
    }

    pub async fn update_ticket(&self, _ctx: Ctx, id: u64, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut tickets_store = self.tickets_store.lock().unwrap();
        let ticket = tickets_store
            .get_mut(id as usize)
            .ok_or(Error::TicketUpdateNotFound{id})?
            .as_mut()
            .ok_or(Error::TicketUpdateNotFound{id})?;
        ticket.title = ticket_fc.title;
        Ok(ticket.clone())
    }

    pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Result<Ticket> {
        let mut tickets_store = self.tickets_store.lock().unwrap();
        let ticket = tickets_store
            .get_mut(id as usize)
            .and_then(|ticket| ticket.take());

        ticket.ok_or(Error::TicketDeleteNotFound{id})
    }
}
// endregion: --- Model Controller