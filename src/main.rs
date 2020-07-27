use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("arguments sont {:?}", args);
    loop {
        let mut reponse = String::new();
        println!("Veux tu jouer ?");
      
      match  io::stdin()
        .read_line(&mut reponse){
            Ok(n) => {
             
                if(reponse.trim()=="oui"){
                let secret_number = rand::thread_rng().gen_range(0, 100);
                println!("le nombre est {} ",secret_number);
                println!("Guess the number!");

                loop{
                    println!("Please input your guess.");

                    let mut guess = String::new();
                    io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");
                    println!("tu as tapé : {} ",guess);
        
       
                let guess: u32 = match guess.trim().parse(){
                    Ok(num)=> num,
                    Err(ParseIntError)=>{ 
                        println!("erreur de saisie veuillez entrez un nombre svp !"); 
                        continue;
                    },
                };
                
                match guess.cmp(&secret_number) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal =>{ println!("You win!");
                                        break;
                                    }
                    }
         }
    }else{
        println!("au revoir");
    break;
    }
} Err(error)=>{
    
}
    }
}
}
