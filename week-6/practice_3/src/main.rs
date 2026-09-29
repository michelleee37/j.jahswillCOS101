fn main() {
   let name1 = "Jahdinma Jahswill";
   println!("My name is {}", name1);

   //to find and then replace
   let name2 = name1.replace("Michelle");
   println!("I preferred to be called {}", name2);
   let faculty = "Faculty of Science and Technology";

   //find and replace
   let school = faculty.replace("Faculty", "School");
   println!("I am a student of the {}", school);
}
