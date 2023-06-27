use axum::routing::{get, put, delete, post};
use axum::{Json, Router};
use axum::extract::{State, Path};
use axum::http::StatusCode;

use crate::ctx::Ctx;
use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;

// Routes for ModelController
pub fn routes(mc: ModelController) -> Router {
    axum::Router::new()
        .route("/tickets", post(handler_create_ticket))
        .route("/tickets", get(handler_list_tickets))
        .route("/tickets/:id", get(handler_get_ticket))
        .route("/tickets/:id", put(handler_update_ticket))
        .route("/tickets/:id", delete(handler_delete_ticket))
        .with_state(mc)
}

// region: --- Rest Handlers

// Handler for create tickets
pub async fn handler_create_ticket(
    State(model_controller): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - handler_create_ticket", "HANDLER");
    let ticket = model_controller.create_ticket(ctx, ticket_fc).await?;
    Ok(Json(ticket))
}

// Handler for list tickets
pub async fn handler_list_tickets(
    State(model_controller): State<ModelController>,
    ctx: Ctx,
) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - handler_list_tickets ", "HANDLER");
    let tickets = model_controller.list_tickets(ctx).await?;
    Ok(Json(tickets))
}

// Handler for get ticket
pub async fn handler_get_ticket(
    Path(id): Path<u64>,
    State(model_controller): State<ModelController>,
    ctx: Ctx,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - handler_get_ticket ", "HANDLER");
    let ticket = model_controller.get_ticket(ctx, id).await?;
    Ok(Json(ticket))
}

// Handler for update ticket
pub async fn handler_update_ticket(
    Path(id): Path<u64>,
    State(model_controller): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - handler_update_ticket ", "HANDLER");
    let ticket = model_controller.update_ticket(ctx, id, ticket_fc).await?;
    Ok(Json(ticket))
}

// Handler for delete ticket
pub async fn handler_delete_ticket(
    Path(id): Path<u64>,
    State(model_controller): State<ModelController>,
    ctx: Ctx,
) -> Result<StatusCode> {
    println!("->> {:<12} - handler_delete_ticket ", "HANDLER");
    model_controller.delete_ticket(ctx, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// endregion: --- Rest Handlers