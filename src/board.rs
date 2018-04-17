use log;

#[derive(Debug)]
pub struct Board {
   pub tiles: Vec<u32>,
   pub human: u8,
   pub robot: u8,
   pub neutral: u8,
   pub minimum_step: u32
}

impl Board {
   pub fn new(tiles: Vec<u32>) -> Board {
      Board {
         tiles: tiles,
         human: 1,
         robot: 2,
         neutral: 0,
         minimum_step: 0
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

   pub fn guess(&mut self) -> String {
      match self.is_playable() {
         Some(_x) => {
            self.do_calculation().to_string()
         },
         None => {
            //log("no move");
            return String::from("-1")
         }
      }
   }

   pub fn get_size(&self) -> u32 {
      (self.tiles.len() as f32 ).sqrt() as u32
   }

   // ? ? ?
   // - - -
   // - - -
   fn horizontal(&self, row :u32) -> Option<u32> {
      //log(&format!("row {}", row.to_string()));
      let x = row*self.get_size();
      let max = x + self.get_size();
      let mut counter = self.get_size() as u32;

      for i in x..max {
         //log(&format!("tiles {}", i));
         let tile_to_check = self.tiles[i as usize];
         // this column can't be claim by human
         if tile_to_check == self.robot as u32 {
            //log(&format!("robot already here"));
            return None;
         }

         // human already claimed a tile from this column
         // reduce the counter
         if tile_to_check == self.human as u32 {
            counter -= 1;
         }
      }
      //log(&format!("edge => {}", counter.to_string()));
      Some(counter)
   }

   // ? - -
   // ? - -
   // ? - -
   fn vertical(&self, column :u32) -> Option<u32> {
      //log(&format!("column {}", column.to_string()));
      let max = self.get_size();
      let mut iteration = 0;
      let mut next = column;
      let mut counter = self.get_size();

      while next <= self.tiles.len() as u32 - 1 {
         //log(&format!("tiles {}", next));
         let tile_to_check = self.tiles[next as usize];

         // this row can't be claim by human
         if tile_to_check == self.robot as u32 {
            return None;
         }

         // human already claimed a tile from this row
         // reduce the counter
         if tile_to_check == self.human as u32 {
            counter -= 1;
         }

         iteration += 1;
         next = iteration * max;
      }

      Some(counter)
   }

   // ? - ?
   // _ ? -
   // ? - ?
   fn diagonal(&self) -> Option<u32> {
      let max = self.get_size();
      let mut iteration = 0;
      let mut counter = self.get_size();
      let mut next = 0;

      while next <= self.tiles.len() as u32 - 1 {
         //log(&format!("tiles {}", next));
         let tile_to_check = self.tiles[next as usize];

         // this row can't be claim by human
         if tile_to_check == self.robot as u32 {
            return None;
         }

         // human already claimed a tile from this row
         // reduce the counter
         if tile_to_check == self.human as u32 {
            counter -= 1;
         }

         iteration += 1;
         next = (max + 1) * iteration
      }


      iteration = 0;
      counter = self.get_size();
      next = max - 1;
      while next <= self.tiles.len() as u32 - 1 {
         //log(&format!("tiles {}", next));
         let tile_to_check = self.tiles[next as usize];

         // this row can't be claim by human
         if tile_to_check == self.robot as u32 {
            return None;
         }

         // human already claimed a tile from this row
         // reduce the counter
         if tile_to_check == self.human as u32 {
            counter -= 1;
         }

         iteration += 1;
         next += max -1;
      }

      Some(counter)
   }

   fn is_playable(&self) -> Option<usize> {
      self.tiles.iter().rposition(|&x| x == self.neutral as u32)
   }

   // check every tile
   // count the least step that human need to take if he claim this tile
   fn do_calculation(&mut self) -> u32 {
      let mut minimum_index = self.is_playable().unwrap() as u32;
      self.minimum_step = self.is_playable().unwrap() as u32;
      for i in 0..self.get_size() {
         //log(&i.to_string());
         for j in 0..self.get_size() {
            //log(&j.to_string());
            //check the number of step that human need to claim this row
            //log(&format!("{},{}", i, j));
            if let Some(x) = self.horizontal(i) {
               if x < self.minimum_step {
                  self.minimum_step = x;
                  minimum_index = i * self.get_size() + j * self.get_size();
               }
            }
            //check the number of step that human need to claim this column
            if let Some(x) = self.vertical((i as f64 / self.get_size() as f64).ceil() as u32) {
               if x < self.minimum_step {
                  self.minimum_step = x;
                  minimum_index = i * self.get_size() + j * self.get_size();
               }
            }
            //if i === j then look for diagonal
            //check the number of step that human need to claim this column
            if i == j {
               if let Some(x) = self.diagonal() {
                  if x < self.minimum_step {
                     self.minimum_step = x;
                     minimum_index = i * self.get_size() + j * self.get_size();
                  }
               }
            }
            log(&format!("minimum_step => {}", self.minimum_step));
         }
      }
      minimum_index
   }
}
