use std::io;

fn start(mut points:i32) -> i32 {
    println!("IT'S CAT TIME! CHOOSE YOUR FIGHTER.");
    println!("1. Romad");
    println!("2. Mimi");
    println!("3. Nobi");
    println!("4. Nemi");
    println!("5. Darwina");

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
    } else if cat_choice.trim() == "3" || cat_choice.trim() == "Nobi" {
        points += 50;
        println!("If you stare into the void long enough, the void stares back.\n");
    } else if cat_choice.trim() == "4" || cat_choice.trim() == "Nemi" {
        points += 50;
        println!("Time to eat some flowers and hang out on the kitchen counter even though we're not allowed up there!\n");
    } else if cat_choice.trim() == "5" || cat_choice.trim() == "Darwina" {
        points += 50;
        println!("THE SMOL POTAT\n");
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
    println!("5. Boot scoot");

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
    } else if morning_choice.trim() == "5" {
        points -= 5000;
        println!(":(\n");
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
    println!("5. Gremlin");

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
    } else if day_choice.trim() == "5" {
        points += 100;
        println!("Being a gremlin can be fun\n");
    } else {
        points -=1000;
        println!("that wasn't a choice");
    }

    return points;
}

fn night_choice(mut points: i32)-> i32{
    println!("Ahhhh, night time after a hard day of doing cat stuff. How do you relax after your busy day of cat errands?");
    println!("1. TIME TO SCREAM");
    println!("2. Sleep above my head");
    println!("3. Bite my finger, toes, nose, and chin");
    println!("4. Throw up on the floor");
    println!("5. Hiss at everyone");

    let mut night_choice = String::new();
    io::stdin()
        .read_line(&mut night_choice)
        .expect("Failed to read line");

    if night_choice.trim() == "1" {
        points += 100;
        println!("A lovely choice to ensure I can't sleep.\n");
    } else if night_choice.trim() == "2" {
        points += 1000;
        println!("Awww!\n");
    } else if night_choice.trim() == "3" {
        points -= 2000;
        println!("PLEASE LET ME SLEEP I HAVE WORK TOMORROW\n");
    } else if night_choice.trim() == "4" {
        points -= 200;
        println!("I wish you wouldn't do this\n");
    } else if night_choice.trim() == "5" {
        points -= 100;
        println!("This won't earn you any friends\n");
    } else {
        points -=1000;
        println!("that wasn't a choice");
    }

    return points;
}

fn press_any_key(message: &str) {
    println!("{message}");
    let mut any_key = String::new();
    io::stdin()
        .read_line(&mut any_key)
        .expect("Failed to read line");
}

fn main() {
    let mut points = 0;
    points = start(points);
    press_any_key("Press any key to continue");
    points = morning_choice(points);
    press_any_key("Press any key to continue");
    points = day_choice(points);
    press_any_key("Press any key to continue");
    points= night_choice(points);

    press_any_key("Press any key to get your cat score!");
    println!("You earned {points} points");
}
