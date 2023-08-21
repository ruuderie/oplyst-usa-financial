use axum::routing::{delete, get, post};
use axum::{Json, Router, extract::{Path, Extension, Query}, extract::{State,FromRef}};
use crate::{Result};

use crate::model::{ModelController, Ticket, TicketForCreate};

//example
#[derive(Clone, FromRef)]
struct AppState {
    model: ModelController,
}

pub fn routes(mc: ModelController) -> Router {
    let app_state = AppState { model: mc };

    Router::new()
        .route("/tickets",post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(app_state)
}

async fn create_ticket(
    State(model): State<ModelController>,
    ctz: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("-->> {:<12} - create_ticket", "HANDLER");
    let ticket = model.create_ticket(ctx,ticket_fc).await?;
    Ok(Json(ticket))
}
async fn list_tickets(State(model): State<ModelController>,ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!("-->> {:<12} - list_tickets", "HANDLER");
    let tickets = model.list_tickets(ctx).await?;
    Ok(Json(tickets))
}
async fn delete_ticket(
    State(model): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u32>,
) -> Result<Json<Ticket>> {
    println!("-->> {:<12} - delete_ticket", "HANDLER");
    let ticket = model.delete_ticket(ctx,id).await?;
    Ok(Json(ticket))
}