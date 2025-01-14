// src/routes.rs

use rocket::http;
use rocket::response;
use rocket::request;
use rocket::State;
use rocket_contrib::json::Json;

use crate::StdErr;
use crate::db::Db;
use crate::models::{Board, CreateBoard, BoardSummary, Card, CreateCard, UpdateCard};

// board routes

#[rocket::get("/boards")]
fn boards(db: State<Db>) -> Result<Json<Vec<Board>>, StdErr> {
    db.boards().map(Json)
}

#[rocket::post("/boards", data = "<create_board>")]
fn create_board(db: State<Db>, create_board: Json<CreateBoard>) -> Result<Json<Board>, StdErr> {
    db.create_board(create_board.0).map(Json)
}

#[rocket::get("/boards/<board_id>/summary")]
fn board_summary(db: State<Db>, board_id: i64) -> Result<Json<BoardSummary>, StdErr> {
    db.board_summary(board_id).map(Json)
}

#[rocket::delete("/boards/<board_id>")]
fn delete_board(db: State<Db>, board_id: i64) -> Result<(), StdErr> {
    db.delete_board(board_id)
}

// card routes

#[rocket::get("/boards/<board_id>/cards")]
fn cards(db: State<Db>, board_id: i64) -> Result<Json<Vec<Card>>, StdErr> {
    db.cards(board_id).map(Json)
}

#[rocket::post("/cards", data = "<create_card>")]
fn create_card(db: State<Db>, create_card: Json<CreateCard>) -> Result<Json<Card>, StdErr> {
    db.create_card(create_card.0).map(Json)
}

#[rocket::patch("/cards/<card_id>", data = "<update_card>")]
fn update_card(
    db: State<Db>,
    card_id: i64,
    update_card: Json<UpdateCard>,
) -> Result<Json<Card>, StdErr> {
    db.update_card(card_id, update_card.0).map(Json)
}

#[rocket::delete("/cards/<card_id>")]
fn delete_card(db: State<Db>, card_id: i64) -> Result<(), StdErr> {
    db.delete_card(card_id)
}

// single public function which returns all API routes

pub fn api() -> Vec<rocket::Route> {
    rocket::routes![
        boards,
        create_board,
        board_summary,
        delete_board,
        cards,
        create_card,
        update_card,
        delete_card,
    ]
}
