use std::io;

fn start(mut points:i32) -> i32 {
    println!("IT'S CAT TIME! CHOOSE YOUR FIGHTER.");
    println!("1. Romad");
    println!("2. Mimi");

    let mut cat_choice = String::new();

    io::stdin()
        .read_line(&mut cat_choice)
        .expect("Failed to read line");

    if cat_choice.trim() == "1" || cat_choice.trim() == "Romad" {
        points += 50;
        println!("Congratulations on being an orange boi for the day!\n");
    } else if cat_choice.trim() == "2" || cat_choice.trim() == "Mimi" {
        points += 50;
        println!("Congratulations on deciding to be spicy for the day. Enjoy biting my feet at 3am while I sleep.\n");
    } else {
        points -= 100;
        println!("You didn't choose a cat. But whatever.\n");
    }

    return points;
}

fn morning_choice(mut points:i32)-> i32{
    println!("You wake up after a long night of doing cat stuff. What is your first move?");
    println!("1. Do a little squeak at me as you wake up on the pillow above my head");
    println!("2. Bite my toes while I try to sleep");
    println!("3. Scream for wet food");
    println!("4. Do nothing");

    let mut morning_choice = String::new();
    io::stdin()
        .read_line(&mut morning_choice)
        .expect("Failed to read line");

    if morning_choice.trim() == "1" {
        points += 100;
        println!("That sounds like a very orange boi move\n");
    } else if morning_choice.trim() == "2" {
        points -= 1000;
        println!("Why would you do this\n");
    } else if morning_choice.trim() == "3" {
        points += 50;
        println!("You don't need wetsies in the morning\n");
    } else if morning_choice.trim() == "4" {
        points -= 100;
        println!("That sounds pretty boring\n");
    } else {
        points -=1000;
        println!("That wasn't a choice!");
    }
    return points;

}

fn day_choice(mut points: i32)-> i32{
    println!("You have some time in your busy schedule to do cat stuff. What do you do?");
    println!("1. TIME TO SCREAM");
    println!("2. Throw up on the floor");
    println!("3. Play with the cat toy");
    println!("4. Sleep");

    let mut day_choice = String::new();
    io::stdin()
        .read_line(&mut day_choice)
        .expect("Failed to read line");

    if day_choice.trim() == "1" {
        points += 100;
        println!("Screaming can be fun sometimes.\n");
    } else if day_choice.trim() == "2" {
        points -= 1000;
        println!("Now I need to clean this up...\n");
    } else if day_choice.trim() == "3" {
        points += 50;
        println!("Honing your hunting instincts is good.\n");
    } else if day_choice.trim() == "4" {
        points += 100;
        println!("Sleeping is healthy. Plus, it means you're not throwing up on the floor\n");
    } else {
        points -=1000;
        println!("that wasn't a choice");
    }

    return points;
}


fn main() {
    let mut points = 0;
    points = start(points);
    points = morning_choice(points);
    points = day_choice(points);

    println!("Press any key to get your cat score!");

    let mut any_key = String::new();
    io::stdin()
        .read_line(&mut any_key)
        .expect("Failed to read line");

    println!("You earned {points} points");
}
