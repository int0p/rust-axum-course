use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

//region:  --- Ticket Types
#[derive(Clone, Debug, Serialize)]
pub struct Ticket{
    pub id: u64,
    pub title: String,
}

/// 사용자가 티켓을 생성할 때 클라이언트로 받는 데이터 (title)
/// id는 클라이언트에서 생성하는것이 아닌 서버에서 생성되므로 포함하지 않는다.
/// 클라이언트에서 받은 JSON데이터를 Rust 데이터 구조로 Deserialize 한다. 
#[derive(Deserialize)]
pub struct TicketForCreate{
    pub title: String,
}
//endregion:  --- Ticket Types

// region:      --- Model Controller
#[derive(Clone)]
pub struct ModelController{
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

/// Constructor
/// Arc::default()
///     Arc::default()는 Arc::new(T::default())와 동일하다. 
///     즉, T 타입의 기본값을 새로운 Arc로 감싸서 반환한다. 
///     해당 구조체에서 tickets_store의 타입은 Arc<Mutex<Vec<Option<Ticket>>>>이므로, 
///     T는 Mutext로 감싼 빈 벡터이다. 
impl ModelController{
    pub async fn new() -> Result<Self>{
        Ok(Self{
            tickets_store: Arc::default(),
        })
    }
}

// CRUD Implementation
impl ModelController{
    pub async fn create_ticket(&self, ticket_fc:TicketForCreate)->Result<Ticket>{
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;
        let ticket = Ticket{
            id,
            title: ticket_fc.title,
        };
        store.push(Some(ticket.clone()));  

        Ok(ticket)
    }

    pub async fn list_tickets(&self)->Result<Vec<Ticket>>{
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t|t.clone()).collect();

        Ok(tickets)
    }

    ///         let ticket = store.get_mut(id as usize).and_then(|t| t.take());
    /// get_mut(id): id에 대한 가변참조를 Option으로 감싸 반환한다. 만일 id가 벡터의 범위를 벗어나면 None을 반환한다.
    /// Option.and_then(|t| t.take()): Option이 Some일 때만 closure를 실행한다.
    ///    이때, t는 get_mut(id)에서 반환한 Option<&mut Ticket> 타입이다.
    ///   take(): Option<T> 타입의 값을 Option::None으로 변경하고, 원래의 값을 반환한다.
    ///     즉, 값이 있던 자리를 비운다. (store에서 제거한다.)
    pub async fn delete_ticket(&self, id:u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());
         
        ticket.ok_or(Error::TicketDeleteFailIdNotFound{id})
    }


}
// endregion:      --- Model Controller