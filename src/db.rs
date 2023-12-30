use diesel::prelude::*;
use diesel::PgConnection;

use crate::schema::boards;
use crate::schema::cards;

use crate::models::{
    Board,
    CreateBoard,
    Card,
    Status,
    BoardSummary,
    StatusCount,
};

fn get_connection() -> PgConnection {
    dotenv::dotenv().unwrap();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url).unwrap()
}

fn create_board(conn : &PgConnection, board : CreateBoard) -> Board {
    diesel::insert_into(boards::table)
    .values(&board)
    .get_result(conn)
    .unwrap()
}


fn get_all_boards(conn : &PgConnection) -> Vec<Board> {
    boards::table
    .order_by(boards::created_at.asc())
    .load::<Board>(conn)
    .unwrap()
}

fn get_board_by_id(conn : &PgConnection, id : i32) -> Option<Board> {
    boards::table
    .filter(boards::id.eq(id))
    .first(conn)
    .unwrap()
}

fn get_board_by_name(conn : &PgConnection, name : &str) -> Vec<Board> {
    boards::table
    .filter(boards::name.eq(name))
    .load(conn)
    .unwrap()
}

fn get_board_name_contains(conn : &PgConnection, name : &str) -> Vec<Board> {
    boards::table
    .filter(boards::name.like(format!("%{}%", name)))
    .load(conn)
    .unwrap()
}

fn get_recent_boards(conn : &PgConnection) -> Vec<Board> {
    let past_day = chrono::Utc::now() - chrono::Duration::days(1);
    boards::table
    .filter(boards::created_at.ge(past_day))
    .order_by(boards::created_at.desc())
    .load(conn)
    .unwrap()
}

fn get_recent_boards_name_contains(conn : &PgConnection, name : &str) -> Vec<Board> {
    let past_day = chrono::Utc::now() - chrono::Duration::days(1);
    let contains = format!("%{}%", name);
    boards::table
    .filter(boards::name.ilike(contains))
    .filter(boards::created_at.ge(past_day))
    .order_by(boards::created_at.desc())
    .load(conn)
    .unwrap()
}

fn get_recent_boards_or_name_contains(conn : &PgConnection, name : &str) -> Vec<Board> {
    let past_day = chrono::Utc::now() - chrono::Duration::days(1);
    let contains = format!("%{}%", name);
    let predicate = boards::name.ilike(contains).or(boards::created_at.ge(past_day));
    boards::table
    .filter(predicate)
    .order_by(boards::created_at.desc())
    .load(conn)
    .unwrap()
}

fn get_board_summary(conn : &PgConnection, board_id : i32) -> BoardSummary {
    diesel::sql_query(
        format!(
            "SELECT COUNT(*), status FROM cards WHERE cards.board_id = {} GROUP BY status", board_id
        ))
    .load::<StatusCount>(conn)
    .unwrap()
    .into()
}

fn delete_all_boards(conn: &PgConnection) {
    diesel::delete(boards::table)
        .execute(conn)
        .unwrap();
}

fn delete_board_by_id(conn: &PgConnection, board_id: i64) {
    diesel::delete(boards::table.filter(boards::id.eq(board_id)))
        .execute(conn)
        .unwrap();
}

fn create_card(conn : &PgConnection, card : Card) -> Card {
    diesel::insert_into(cards::table)
    .values(&card)
    .get_result(conn)
    .unwrap()
}

fn update_card(conn: &PgConnection, card_id: i64, update_card: UpdateCard) -> Card {
    diesel::update(cards::table.filter(cards::id.eq(card_id)))
        .set(update_card)
        .get_result(conn)
        .unwrap()
}

fn get_all_cards(conn : &PgConnection) -> Vec<Card> {
    cards::table
    .order_by(cards::created_at.asc())
    .load::<Card>(conn)
    .unwrap()
}

fn get_card_by_board_id(conn : &PgConnection, board_id : i32) -> Vec<Card> {
    cards::table
    .filter(cards::board_id.eq(board_id))
    .order_by(cards::created_at.asc())
    .load(conn)
    .unwrap()
}

fn get_card_by_status(conn : &PgConnection, status : Status) -> Vec<Card> {
    cards::table
    .filter(cards::status.eq(status))
    .order_by(cards::created_at.asc())
    .load(conn)
    .unwrap()
}

fn delete_all_cards(conn: &PgConnection) {
    diesel::delete(cards::table)
        .execute(conn)
        .unwrap();
}

fn delete_card_by_id(conn: &PgConnection, card_id: i64) {
    diesel::delete(cards::table.filter(cards::id.eq(card_id)))
        .execute(conn)
        .unwrap();
}

fn delete_cards_by_board_id(conn: &PgConnection, board_id: i64) {
    diesel::delete(cards::table.filter(cards::board_id.eq(board_id)))
        .execute(conn)
        .unwrap();
}

fn delete_done_cards_by_board_id(conn: &PgConnection, board_id: i64) {
    diesel::delete(
        cards::table
            .filter(cards::board_id.eq(board_id))
            .filter(cards::status.eq(Status::Done)),
    )
    .execute(conn)
    .unwrap();
}
