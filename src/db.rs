use diesel::prelude::*;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};


use crate::StdError;


use crate::models::CreateCard;
use crate::schema::boards;
use crate::schema::cards;

use crate::models::{
    Board,
    CreateBoard,
    Card,
    UpdateCard,
    Status,
};

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct Db {
    pool : PgPool,
}

impl Db {
    pub fn connect() -> Result<Self, StdError> {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool = Pool::new(manager)?;
        Ok(Db { pool })
    }

    fn create_board(&self, board: CreateBoard) -> Result<Board, StdError> {
        let mut conn = self.pool.get()?;

        let board = diesel::insert_into(boards::table)
            .values(&board)
            .get_result(&mut conn)?;

        Ok(board)
    }


    fn get_all_boards(&self) -> Result<Vec<Board>, StdError> {
        let mut conn = self.pool.get()?;
        Ok(boards::table
        .order_by(boards::created_at.asc())
        .load::<Board>(&mut conn)?)
    }

    fn get_board_by_id(&self, id: i64) -> Result<Board, StdError> {
        let mut conn = self.pool.get()?;

        boards::table
            .filter(boards::id.eq(id))
            .first(&mut conn)
            .map_err(|e| e.into())
    }

    fn get_board_by_name(&self, name : &str) -> Result<Vec<Board>, StdError> {
        let mut conn = self.pool.get()?;

        Ok(boards::table
        .filter(boards::name.eq(name))
        .load(&mut conn)?)
    }

    fn get_board_name_contains(&self, name : &str) -> Result<Vec<Board>, StdError> {
        let mut conn = self.pool.get()?;

        Ok(boards::table
        .filter(boards::name.like(format!("%{}%", name)))
        .load(&mut conn)?)
    }

    fn get_recent_boards(&self) -> Result<Vec<Board>, StdError> {
        let mut conn = self.pool.get()?;

        let past_day = chrono::Utc::now() - chrono::Duration::days(1);
        let boards = boards::table
        .filter(boards::created_at.ge(past_day))
        .order_by(boards::created_at.desc())
        .load(&mut conn)?;
        Ok(boards)
    }

    fn get_recent_boards_name_contains(&self, name : &str) -> Result<Vec<Board>, StdError> {
        let mut conn = self.pool.get()?;
        let past_day = chrono::Utc::now() - chrono::Duration::days(1);
        let contains = format!("%{}%", name);
        let boards = boards::table
        .filter(boards::name.ilike(contains))
        .filter(boards::created_at.ge(past_day))
        .order_by(boards::created_at.desc())
        .load(&mut conn)?;
        Ok(boards)
    }

    fn get_recent_boards_or_name_contains(&self, name : &str) -> Result<Vec<Board>, StdError> {
        let mut conn = self.pool.get()?;

        let past_day = chrono::Utc::now() - chrono::Duration::days(1);
        let contains = format!("%{}%", name);
        let predicate = boards::name.ilike(contains).or(boards::created_at.ge(past_day));
        let boards = boards::table
        .filter(predicate)
        .order_by(boards::created_at.desc())
        .load(&mut conn)?;
        Ok(boards)
    }

    fn delete_all_boards(&self) -> Result<(), StdError> {
        let mut conn = self.pool.get()?;
        diesel::delete(boards::table)
            .execute(&mut conn)?;
        Ok(())
    }

    fn delete_board_by_id(&self, board_id: i64) -> Result<(), StdError> {
        let mut conn = self.pool.get()?;
        diesel::delete(boards::table.filter(boards::id.eq(board_id)))
            .execute(&mut conn)?;
        Ok(())
    }

    fn create_card(&self, card : CreateCard) -> Result<Card, StdError> {
        let mut conn = self.pool.get()?;
        let card = diesel::insert_into(cards::table)
        .values(&card)
        .get_result(&mut conn)?;
        Ok(card)
    }

    fn update_card(&self, card_id: i64, update_card: UpdateCard) -> Result<Card, StdError> {
        let mut conn = self.pool.get()?;
        let card = diesel::update(cards::table.filter(cards::id.eq(card_id)))
            .set(update_card)
            .get_result(&mut conn)?;
        Ok(card)
    }

    fn get_all_cards(&self) -> Result<Vec<Card>, StdError> {
        let mut conn = self.pool.get()?;
        let cards = cards::table
        .order_by(cards::created_at.asc())
        .load::<Card>(&mut conn)?;
        Ok(cards)
    }

    fn get_card_by_board_id(&self, board_id: i64) -> Result<Vec<Card>, StdError> {
        let mut conn = self.pool.get()?;
        let cards = cards::table
            .filter(cards::board_id.eq(board_id))
            .order_by(cards::created_at.asc())
            .load(&mut conn)?;
        Ok(cards)
    }



    fn get_card_by_status(&self, status : Status) -> Result<Vec<Card>, StdError> {
        let mut conn = self.pool.get()?;
        let cards = cards::table
        .filter(cards::status.eq(status))
        .order_by(cards::created_at.asc())
        .load(&mut conn)?;
        Ok(cards)
    }

    fn delete_all_cards(&self) -> Result<(), StdError> {
        let mut conn = self.pool.get()?;
        diesel::delete(cards::table)
            .execute(&mut conn)?;
        Ok(())
    }

    fn delete_card_by_id(&self, card_id: i64) -> Result<(), StdError> {
        let mut conn = self.pool.get()?;
        diesel::delete(cards::table.filter(cards::id.eq(card_id)))
            .execute(&mut conn)?;
        Ok(())
    }

    fn delete_cards_by_board_id(&self, board_id: i64) -> Result<(), StdError> {
        let mut conn = self.pool.get()?;
        diesel::delete(cards::table.filter(cards::board_id.eq(board_id)))
            .execute(&mut conn)?;
        Ok(())
    }

    fn delete_done_cards_by_board_id(&self, board_id: i64) -> Result<(), StdError> {
        let mut conn = self.pool.get()?;
        diesel::delete(
            cards::table
                .filter(cards::board_id.eq(board_id))
                .filter(cards::status.eq(Status::Done)),
        )
        .execute(&mut conn)?;
        Ok(())
    }

}
