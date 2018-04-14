use log;

#[derive(Debug)]
pub struct Board {
   pub tiles: Vec<u32>,
   pub human: u8,
   pub robot: u8,
   pub neutral: u8
}

impl Board {
   pub fn new(tiles: Vec<u32>) -> Board {
      Board {
         tiles: tiles,
         human: 1,
         robot: 2,
         neutral: 0
      }
   }

   pub fn log(&self) -> String {
      let size = self.get_size();
      let chunks = self.tiles.chunks(size as usize);
      let mut logs = String::from("");
      let new_line = "\n".to_string();
      for chunk in chunks {
         for t in chunk.iter() {
            let s_t = format!("{} ", *t);
            logs.push_str(&s_t);
         }
         logs.push_str(&new_line);
      }

      logs
   }

   pub fn guess(&self) -> i64 {
      match self.tiles.iter().position(|&x| x == 0) {
         Some(x) => {
            self.do_calculation()
         },
         None => {
            //log("no move");
            return -1
         }
      }
   }

   pub fn get_size(&self) -> u32 {
      (self.tiles.len() as f32 ).sqrt() as u32
   }

   fn horizontal(&self) {

   }

   fn vertical(&self) {

   }

   fn diagonal(&self) {

   }

   fn is_playable(&self) -> Option<usize> {
      self.tiles.iter().position(|&x| x == 0)
   }

   fn do_calculation(&self) -> i64 {
      let default = self.tiles.iter().rposition(|&d| d == self.neutral as u32).unwrap();
      for i in 0..self.get_size() {
         //log(&i.to_string());
         for j in 0..self.get_size() {
            //log(&j.to_string());
            //look for horizontal
            //look for vertical
            //if i === j then look for diagonal
         }
      }

      default as i64
   }
}
