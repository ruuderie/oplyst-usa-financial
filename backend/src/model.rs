use crate::{Error,Result, web};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Clone)]
pub struct Ticket {
    pub cid: u32,
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: String,
}
#[derive(Debug, Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}
#[derive(Debug, Clone)]
pub struct ModelController {
//usually it's a db
    pub tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,

}

impl ModelController {
    pub async fn new() -> Result<Self>{
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

impl ModelController {
    pub async fn create_ticket(&self, ctx: Ctx,ticket: TicketForCreate) -> Result<Ticket> {
        let mut tickets_store = self.tickets_store.lock().unwrap();
        let id = tickets_store.len() as u32;
        let ticket = Ticket {
            id,
            cid: ctx.user_id(),
            title: ticket.title,
            description: "".to_string(),
            status: "open".to_string(),
        };
        tickets_store.push(Some(ticket.clone()));
        Ok(ticket)
    }
    pub async fn get_ticket(&self,_ctx: Ctx, id: u32) -> Result<Ticket> {
        let tickets_store = self.tickets_store.lock().unwrap();
        let ticket = tickets_store.get(id as usize).unwrap().clone().unwrap();
        Ok(ticket)
    }
    pub async fn update_ticket(&self,_ctx: Ctx, id: u32, ticket: Ticket) -> Result<Ticket> {
        let mut tickets_store = self.tickets_store.lock().unwrap();
        let ticket = tickets_store.get_mut(id as usize).unwrap().clone().unwrap();
        Ok(ticket)
    }
    pub async fn delete_ticket(&self,_ctx: Ctx, id: u32) -> Result<Ticket> {
        let mut tickets_store = self.tickets_store.lock().unwrap();
        let ticket = tickets_store.get_mut(id as usize).and_then(|t| t.take());
        ticket.ok_or(Error::TicketDeleteFailIdNotFound {id})
    }
    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        let tickets_store = self.tickets_store.lock().unwrap();
        let tickets = tickets_store.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }
}