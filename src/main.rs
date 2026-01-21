use std::cmp::Ordering;
use std::fs;
use prompted::input;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct CollatzRes{
	basenum:u64,
    collatznum:u64
}

fn collatz(inputnum: u64) -> u64{
    let basenum:u64 = inputnum;
	let mut interimnum:u64 = basenum;
	let mut collatznum:u64 = 0;
    loop{
		collatznum += 1;
        if interimnum %2 == 0{
			interimnum = interimnum / 2;
	    }
		else{
			interimnum = (3*interimnum) + 1;
		}
		if interimnum == 1{
			break;
		}
    }
    return collatznum;	
}

fn collatz_verbose(inputnum: u64) -> u64{
    let basenum:u64 = inputnum;
	let mut interimnum:u64 = basenum;
	let mut collatznum:u64 = 0;
    loop{
		collatznum += 1;
        if interimnum %2 == 0{
			interimnum = interimnum / 2;
	    }
		else{
			interimnum = (3*interimnum) + 1;
		}
		if interimnum == 1{
			break;
		}
		println!("Loop {} interimvalue: {}",collatznum,interimnum);
    }
    return collatznum;	
}

fn collatz_range(startrng: u64, endrng: u64) -> Vec<CollatzRes>{
	let mut collatzvec:Vec<CollatzRes> = Vec::new();
	println!("Got {} and {}",startrng,endrng);
	for i in startrng..endrng {
		//println!("Loop {}",i);
		collatzvec.push(CollatzRes{basenum: i,collatznum: collatz(i)});
	}
	return collatzvec;
}

fn direct_collatz(verbose: bool){
	let inputstr = input!("Please input the testnumber for collatz conjecture\n (or 'q' to quit)\n>");
	if inputstr.to_uppercase() == "Q"{
		return;
	}
	else{
		let inputint: u64 = inputstr.parse::<u64>().expect("Your input could not be converted to a unsigned integer");
		if verbose{
			let result = collatz_verbose(inputint);
			println!("This is the result:{} for input {}",result,inputstr);
		}
		else{
		    println!("This is the result:{} for input {}",collatz(inputint),inputstr);
		}
	}
}

fn range_collatz(){
	let mut inputstr = input!("Please input start of the range for collatz conjecture\n (or 'q' to quit)\n>");
	let rng1int: u64;
	let rng2int: u64;
	if inputstr.to_uppercase() == "Q"{
		return;
	}
	else{
		rng1int = inputstr.parse::<u64>().expect("Your input could not be converted to a unsigned integer");
	}
	inputstr = input!("Please input end of the range for collatz conjecture\n (or 'q' to quit)\n>");
	if inputstr.to_uppercase() == "Q"{
		return;
	}
	else{
		rng2int = inputstr.parse::<u64>().expect("Your input could not be converted to a unsigned integer");
	}
	let collatzvec:Vec<CollatzRes> = collatz_range(rng1int, rng2int);
	println!("Length of the vector: {}",collatzvec.len());
	let outputstr = serde_json::to_string_pretty(&collatzvec).expect("Could not convert the vector of CollatzRes to json");
	// std::fs::write will overwrite the file if it already exists
    fs::write("data.json", outputstr).expect("Failed to write JSON to file");
	//for col in collatzvec{
	//	println!("This is the result:{} for input {}",col.collatznum,col.basenum);
	//}
}

fn _get_u16_from_user(maxnum:u16) -> u16 {
    loop {
        let raw_input = input!("Please enter a number <= {}\n:",maxnum);
        // .trim() is important to remove the newline character (\n) 
        match raw_input.trim().parse::<u16>() {
            Ok(num) => {
				if num <= maxnum {
                    return num;
				}
				else{
					 println!("Error: '{}' is not <= {}. Please try again.", raw_input.trim(),maxnum);
					 continue;
				}
            }
            Err(_) => {
                // If parsing fails, print a warning and the loop repeats
                println!("Error: '{}' is not a valid u16. Please try again.", raw_input.trim());
            }
        }
    }
}

fn play_collatz(){
	let mut budget:u16 = 100;
	let mut score:u16 = 0;
	let mut used:Vec<u16> = Vec::new();
	let mut test:u16 = 0;
	let mut bigtest;
	loop{
		test = _get_u16_from_user(budget);
	    bigtest = u64::from(test);
		score += u16::try_from(collatz(bigtest)).expect("Value was too large to fit into a u16!");
		used.push(test);
		budget -= test;
	    println!("Collatznum = {}, from basenum {}",collatz(bigtest), bigtest);
		println!("Score = {}, Budget {}",score, budget);
		if score > 100 || budget <= 1{
			break;
		}
	}
	println!("Finished");
	println!("Score = {}, Budget {}",score, budget);
}

fn game_collatz(){
	let inputstr = gamemenu();
	if inputstr.to_uppercase() == "Q"{
			return;
		}
	else if inputstr.to_uppercase() == "S"{
        play_collatz()
	}
}

fn cls(){
	clearscreen::clear().expect("failed to clear screen");
}

fn mainmenu() -> String{
	cls();
	let mainmenustr = "Mainmenu
-----------
D)Direct collatznumber
V)Direct collatznumber with steps writen to console
R)Collatznumber over range
G)Collatz Game
Q)Quit";
    println!("{}",mainmenustr);
	let inputstr:String = input!(">");
	return inputstr;
}

fn gamemenu() -> String{
	cls();
	let gamemenustr = "Collatz Game
------------
The rules of the game:
- Pick numbers such that the sum of their collatznumbers
  is as high as possible.
- The numbers you pick are substracted from your budget(100).
- You beat the game by getting a score higher than your initial budget(100).
S)Start
Q)Quit";
    println!("{}",gamemenustr);
	let inputstr:String = input!(">");
	return inputstr;
}

fn anykey(){
	input!("Type Any Key to continue");
}

fn main() {
    println!("Welcome to the collatz guesser");
	loop{
		let inputstr = mainmenu();
		if inputstr.to_uppercase() == "Q"{
			break;
		}
		else if inputstr.to_uppercase() == "D"{
			direct_collatz(false);
			anykey();
		}
		else if inputstr.to_uppercase() == "V"{
			direct_collatz(true);
			anykey();
		}
		else if inputstr.to_uppercase() == "R"{
			range_collatz();
			anykey();
		}
		else if inputstr.to_uppercase() ==	 "G"{
			game_collatz();
			anykey();
		}
		
	}
	
}
