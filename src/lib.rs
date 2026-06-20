pub struct Board {}
pub struct Move {}


impl Board {
      pub fn from_fen(fen: &str) -> Board { unimplemented!() }
      pub fn legal_moves(&self) -> Vec<Move> { unimplemented!() }
      pub fn make_move(&self, mv: &Move) -> Board { unimplemented!() }  // immutable apply → no unmake needed
  }
