use axum::Json;
use axum::extract::{State, Path};

use crate::model::{ModelController, Ticket,TicketForCreate};
use crate::Result;

// region:     --- Model Controller ---REST Handlers
async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
)->Result<Json<Ticket>>{
    println!("->> {:<12} - create_ticket","HANDLER");
    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(
    State(mc): State<ModelController>,
)->Result<Json<Vec<Ticket>>>{
    println!("->> {:<12} - list_tickets","HANDLER");
    let tickets = mc.list_tickets().await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
)->Result<Json<Ticket>>{
    println!("->> {:<12} - delete_ticket","HANDLER");
    let ticket = mc.delete_ticket(id).await?;

    Ok(Json(ticket))
}
// endregion:     --- Model Controller ---REST Handlers
