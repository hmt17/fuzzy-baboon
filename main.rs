//Hannah Tarzian 
//deliverable 7

extern crate rand;

// Use for I/O
use std::io;
//use std::io::Error;
//use to exit
use std::process;
// use for getting random numbers - relies on extern crate rand
use rand::Rng;

#[derive(Copy, Clone)]
enum Choices {
    Rock,
    Paper,
    Scissor
}

struct Results {
	wins : f64,
	losses : f64,
	tie : f64,
	plays: f64,
	rock : i32,
	scissor : i32,
	paper : i32
}

//generate random number to represent rock, paper, scissor
fn pick_random_call() -> u32 {
    let call = rand::thread_rng().gen_range(1,4);
	return call;
}

//checks if char input is good
fn good_input(to_return: String) -> u32 {
		if to_return.trim() == "r"{
			println!("Player chose: Rock");
			return 1;
		}
		else if to_return.trim() == "s" {
			println!("Player chose: Scissor");
			return 2;
		}
		else if to_return.trim() == "p" {
			println!("Player chose: Paper");
			return 3;
		}
		else if to_return.trim() == "q" {
			println!("Player Stats:");
			return 4;
		}
		else {
			return 0;
		}
}


//update results struct for each variable (borrowing function)
fn update_results(good: u32, mut results_new: &mut Results) {
	if good == 1 {
		results_new.rock += 1;
	}
	else if good == 2 {
		results_new.scissor += 1;
	}
	else if good == 3 {
		results_new.paper += 1;
	}
	else if good == 4 {
		quit(results_new);
	}
	else if good == 5 {
		results_new.wins += 1.0;
	}
	else if good == 6 {
		results_new.losses += 1.0;
	}
}

//compare, decide winner each round and update struct
fn compare_choices(c: Choices, computer: u32, result: &mut Results) {
	if computer == 1 {
		match c {
			Choices::Rock => { result.tie +=1.0; println!("Computer played rock. You tied!");},
			Choices::Scissor => { result.losses +=1.0; println!("Computer played rock. You win!");},
			Choices::Paper => { result.wins +=1.0; println!("Computer played rock. You lost!");},
		}
	}
	else if computer == 2 {
		match c {
			Choices::Rock => { result.wins +=1.0; println!("Computer played scissor. You won!");},
			Choices::Scissor => { result.tie +=1.0; println!("Computer played scissor. You tied!");},
			Choices::Paper => { result.losses +=1.0; println!("Computer played scissor. You lost!");},
		}
	}
	else if computer == 3 {
		match c {
			Choices::Rock => { result.losses +=1.0; println!("Computer played paper. You lost!");},
			Choices::Scissor => { result.wins +=1.0; println!("Computer played paper. You win!");},
			Choices::Paper => { result.tie +=1.0; println!("Computer played paper. You tied!");},
		}
	}

}

//quit and display stats
fn quit(results: &mut Results) {
	let  percent_win = (results.wins)/(results.plays)*100.00;
	let  percent_loss = (results.losses)/(results.plays)*100.00;
	let  percent_tie = (results.tie)/(results.plays)*100.00;
	println!("Wins: {} ({:.2}%)", results.wins, percent_win);
	println!("Ties: {} ({:.2}%)", results.tie, percent_tie);
	println!("Losses: {} ({:.2}%)", results.losses, percent_loss);
	println!("Rocks: {}", results.rock);
	println!("Paper: {}", results.paper);
	println!("Scissor: {}", results.scissor);
	process::exit(1);
}

// For each turn, pick r, p, or s
fn main() {
    
	//initialize struct 
	let mut results = Results{wins: 0.0, losses: 0.0, tie: 0.0, plays: 0.0, rock: 0, scissor: 0, paper: 0}; 
	
	//enums
	let c1: Choices = Choices::Rock;
	let c2: Choices = Choices::Paper;
	let c3: Choices = Choices::Scissor;
	
	//reads in input
	let mut good = 0; 	
	while good!=4 {
		while good == 0
		{
			println!("Enter choice");
			let mut to_return = String::new();
			io::stdin()
				.read_line(&mut to_return)
				.expect("FAIL");
			//checks if good	
			good = good_input(to_return);
			update_results(good, &mut results);
		}
		
		//calls for random choice
		let rand_num = pick_random_call();
		
		//update number of plays
		results.plays += 1.0;
		
		//compare random with enum 
		if good == 1 {
			compare_choices(c1, rand_num, &mut results);
		}
		else if good == 2 {
			compare_choices(c2, rand_num, &mut results);
		}
		else if good == 3 {
			compare_choices(c3, rand_num, &mut results);
		}
	}
}
