#[derive(Debug)]
pub struct Card {
    pub id: u32,
    pub winning: Vec<u32>,
    pub chosen: Vec<u32>,
}