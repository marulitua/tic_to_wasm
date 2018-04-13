use log;

#[derive(Debug)]
pub struct Board {
   pub tiles: Vec<u32>
}

impl Board {
   pub fn log(&self) {
      let chunks = self.tiles.chunks(self.get_size() as usize);
      let mut logs = String::from("");
      for chunk in chunks {
         //logs = format!("{}\n{}", logs, String::from(chunk));
      }

      log(&logs);
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
