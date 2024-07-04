pub struct ScreenHandler {
}

impl ScreenHandler {
   pub fn test(self) -> String {
       return "hello, word".to_owned()
   } 
   pub fn name(self, name: String) -> String {
       return name
   }
}
