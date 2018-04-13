#[derive(Debug)]
pub struct Board {
   pub tiles: Vec<u32>
}

impl Board {
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

   pub fn get_size(&self) -> u32 {
      (self.tiles.len() as f32 ).sqrt() as u32
   }
}

impl ToString for Board {

   fn to_string(&self) -> String {
      String::from("this is what board is")
   }

}
