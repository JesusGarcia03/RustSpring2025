fn borrow_to_mut_watchout() {
    let mut word = "UT".to_string(); 
    let ptr_mut = &mut word;
    println!("{ptr_mut}");
    let ptr_mut1 = &mut word;
    println!("{word}");
    //drop(ptr_mut1);

   // println!("{ptr_mut}, {ptr_mut1}");
}

fn main(){
    borrow_to_mut_watchout();
}