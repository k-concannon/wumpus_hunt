mod init;

use std::io;
use rand::Rng;
#[derive(Copy, Clone)]
pub struct Cave{
    x: usize,
    y:usize
}
#[derive(Copy, Clone)]
pub struct Wumpus {
    x: usize,
    y: usize,
    alive: bool
}
#[derive(Copy, Clone)]
pub struct Bats{
    x: usize,
    y: usize
}
#[derive(Copy, Clone)]
pub struct Player{
    x: usize,
    y: usize,
    arrow_count: usize,
    alive: bool,
    death_message: i32

}

#[derive(Copy, Clone)]
pub struct Arrow{
    x: usize,
    y: usize,
    found: bool
}
fn player_location_gen(borrowed_cave: Cave, borrowed_bats: Bats, borrowed_wumpus: Wumpus) -> (usize, usize){
    let mut acceptable_values = Vec::new();
    let mut good_val = false;
    for i in 0..5{
        if i != borrowed_wumpus.x  && i != borrowed_bats.x  && i != borrowed_cave.x {
            acceptable_values.push(i);
            good_val = true;
        }
        else if good_val{
            acceptable_values.push(i-1);
            good_val = false;
        }

    }
    let x = acceptable_values[rand::thread_rng().gen_range(0..acceptable_values.len()-1)];
    let y = acceptable_values[rand::thread_rng().gen_range(0..acceptable_values.len()-1)];
    return (x, y)

}
fn bats_location_gen(borrowed_cave: Cave, borrowed_wumpus: Wumpus) -> (usize, usize) {
    let mut acceptable_values = Vec::new();
    let mut good_val = false;
    for i in 0..5{
        if i != borrowed_wumpus.x &&  i != borrowed_cave.x {
            acceptable_values.push(i);
            good_val = true;
        }
        else if good_val{
            acceptable_values.push(i-1);
            good_val = false;
        }

    }
    let x = acceptable_values[rand::thread_rng().gen_range(0..acceptable_values.len())];
    let y = acceptable_values[rand::thread_rng().gen_range(0..acceptable_values.len())];
    return (x, y)


}
fn cave_location_gen( borrowed_wumpus: Wumpus) -> (usize, usize) {
    let mut acceptable_values = Vec::new();
    let mut good_val = false;
    for i in 0..5{
        if i != borrowed_wumpus.x {
            acceptable_values.push(i);
            good_val = true;
        }
        else if good_val{
            acceptable_values.push(i-1);
            good_val = false;
        }

    }
    let x = acceptable_values[rand::thread_rng().gen_range(0..acceptable_values.len()-1)];
    let y = acceptable_values[rand::thread_rng().gen_range(0..acceptable_values.len()-1)];
    return (x, y)


}


fn arrow_location_gen(borrowed_player: Player, borrowed_cave: Cave, borrowed_bats: Bats, borrowed_wumpus: Wumpus) -> (usize, usize){
    let mut acceptable_values = Vec::new();
    let mut good_val_x = false;
    let mut good_val_y = false;
    for i in 0..=5{
        if i != borrowed_wumpus.x && i != borrowed_bats.x && i != borrowed_cave.x && i != borrowed_player.x{
            acceptable_values.push(i);
            good_val_x = true;
        }
        else if good_val_x{
            acceptable_values.push(i-1);
            good_val_x = false;
        }
        if i != borrowed_wumpus.y && i != borrowed_cave.y && i != borrowed_bats.y && i != borrowed_player.x{
            acceptable_values.push(i);
            good_val_y = true;
        }
        else if good_val_y{
            acceptable_values.push(i-1);
            good_val_y = false;
        }

    }
    let x = acceptable_values[rand::thread_rng().gen_range(0..=5)];
    let y = acceptable_values[rand::thread_rng().gen_range(0..=5)];
    return (x, y)

}
fn fire_arrow(player: &mut Player, mut wumpus: &mut Wumpus, bats: Bats, cave: Cave){
    if player.arrow_count > 0{
        println!("Enter the direction you would like to fire your arrow");
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input.pop();
        if input == "W" || input == "w"{
            if wumpus.x as i32 == player.x as i32 - 1{
                wumpus.alive = false;
            }
            else { move_wumpus(&mut wumpus, *player, bats, cave) }
            player.arrow_count = player.arrow_count - 1;
        }
        else if input == "S" || input == "s"{
            if wumpus.x as i32== player.x as i32 + 1{
                wumpus.alive = false;
            }
            else { move_wumpus(&mut wumpus, *player, bats, cave) }
            player.arrow_count = player.arrow_count - 1;
        }
        else if input == "A" || input == "a"{
            if wumpus.y  as i32 == player.y as i32 - 1{
                wumpus.alive = false;
            }
            else { move_wumpus(&mut wumpus, *player, bats, cave) }
            player.arrow_count = player.arrow_count - 1;
        }
        else if input == "D" || input == "d"{
            if wumpus.y as i32 == player.y as i32 + 1{
                wumpus.alive = false;
            }
            else { move_wumpus(&mut wumpus, *player, bats, cave) }
            player.arrow_count = player.arrow_count - 1;
        }
        else { println!("Invalid Input!!!") }
    }
    else {
        println!("You have no arrows left!!!")
    }
}

fn move_wumpus(wumpus: &mut Wumpus, borrowed_player: Player, borrowed_bats: Bats, borrowed_cave: Cave){
    if (wumpus.x +1 != borrowed_cave.x && wumpus.x + 1 != borrowed_cave.x && wumpus.x +1 != borrowed_player.x && wumpus.x +1 != borrowed_bats.x) && wumpus.x < 4 {
        wumpus.x = wumpus.x +1;

    }
    else if (wumpus.y +1 != borrowed_cave.y && wumpus.y + 1 != borrowed_cave.y && wumpus.y +1 != borrowed_player.y&& wumpus.y +1 != borrowed_bats.y) && wumpus.y < 4 {
        wumpus.y = wumpus.y +1;

    }
    else if (wumpus.x as i32 -1 != borrowed_cave.x as i32 && wumpus.x as i32 - 1 != borrowed_cave.x as i32 && wumpus.x as i32 -1 != borrowed_player.x as i32 && wumpus.x  as i32 -1 != borrowed_bats.x as i32) && wumpus.x > 0 {
        wumpus.x = wumpus.x -1

    }
    else if (wumpus.y as i32 -1 != borrowed_cave.y as i32 && wumpus.y as i32 - 1 != borrowed_cave.y as i32 && wumpus.y as i32 -1 != borrowed_player.y as i32 && wumpus.y  as i32 -1 != borrowed_bats.y as i32) && wumpus.y > 0 {
        wumpus.y = wumpus.y -1;

    }
}
fn base_logic(grid: &mut [[(bool, char); 5]; 5], player: &mut Player, wumpus: Wumpus, cave: Cave, bats: Bats, arrow: &mut Arrow){
    if player.x == wumpus.x && player.y == wumpus.y{
        player.alive = false;
        player.death_message = 1;
    }
    if player.x == bats.x && player.y == bats.y{
        let (x , y) = player_location_gen(cave, bats, wumpus);
        player.x = x;
        player.y = y;
        println!("You were picked up by bats and carried away!!!")
    }
    if player.x == cave.x && player.y == cave.y{
        player.alive = false;
        player.death_message = 2;
    }
    if player.x == arrow.x && player.y == arrow.y && arrow.found == false {
        arrow.found = true;
        player.arrow_count = player.arrow_count + 1;
        println!("You found an arrow");
    }
    for i in 0..5 {
        for j in 0..5{
            if grid[i][j].0 == true{
                grid[i][j].1 = '@';
            }
            if i == player.x && j == player.y{
                grid[i][j].1 = 'o';
                grid[i][j].0 = true;
            }

        }
    }
}

fn grid_print(grid: [[(bool, char); 5]; 5]){
    for i in 0..5 {
        for j in 0..5{
            print!("{}  ", grid[j][i].1);
        }
        println!("")
    }
}
fn player_hint(player: Player, wumpus: Wumpus, bats: Bats, cave: Cave){
    if (player.x == bats.x +1 || player.x as i32 == bats.x as i32 -1 ||player.x == bats.x) && (player.y as i32 == bats.y as i32 -1 || player.y == bats.y +1 || player.y == bats.y) {
        println!("You hear the flapping of bats wings...")

    }
    if (player.x == cave.x +1 || player.x as i32 == cave.x as i32 -1 ||player.x == cave.x) && (player.y as i32 == cave.y as i32 -1 || player.y == cave.y +1 || player.y == cave.y) {
        println!("You see the entrance of a cave nearby...")

    }
    if (player.x == wumpus.x +1 || player.x as i32 == wumpus.x as i32 -1 || player.x == wumpus.x) && (player.y as i32 == wumpus.y as i32 -1 || player.y == wumpus.y +1 || player.y == wumpus.y) {
        println!("You smell a wumpus...")

    }
    println!("You have {} arrows left", player.arrow_count)
}
fn input_logic(player: &mut Player, wumpus: &mut Wumpus, cave: Cave, bats: Bats ,input: String){
    if input == "W" && player.y > 0 || input == "w" && player.y > 0{
        player.y = player.y - 1;
    }
    else if input == "A" && player.x > 0 || input == "a" && player.x > 0{
        player.x = player.x - 1;
    }
    else if input == "S" && player.y < 4 || input == "s" && player.y < 4 {
        player.y = player.y + 1;
    }
    else if input == "D" && player.x < 4 || input == "d" && player.x < 4{
        player.x = player.x + 1;
    }
    else if input == "F" || input == "f"{
        fire_arrow(player, wumpus, bats, cave);
    }
    else{
        println!("Invalid Input!!!")
    }
}
fn death_print(player: Player){
    if player.death_message == 0 {
        println!("You survived and killed the Wumpus!!!");
    }
    else if player.death_message == 1{
        println!("The hungry Wumpus ate you!!!");
    }
    else if player.death_message == 2{
        println!("You fell down a cave and died!!!");
    }
}

fn main_loop(player: &mut Player, wumpus: &mut Wumpus, arrow: &mut Arrow, bats: Bats, cave: Cave,grid: &mut [[(bool, char); 5]; 5] ){
    while player.alive == true && wumpus.alive == true {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input.pop();
        input_logic(player, wumpus, cave, bats, input);
        base_logic(grid, player, *wumpus, cave, bats, arrow);
        grid_print(*grid);
        player_hint(*player, *wumpus, bats, cave);

    }
}

fn main() {

    let mut wumpus = Wumpus {x: rand::thread_rng().gen_range(0..5), y: rand::thread_rng().gen_range(0..5), alive: true };
    let (x, y) = cave_location_gen(wumpus);
    let cave = Cave {x, y};
    let (x, y) = bats_location_gen(cave, wumpus);
    let bats = Bats {x, y};
    let (x, y) = player_location_gen(cave, bats, wumpus);
    let mut player = Player{x, y, arrow_count: 5, alive: true, death_message: 0};
    let (x, y) = arrow_location_gen(player, cave, bats, wumpus);
    let mut arrow = Arrow{x, y, found: false};
    //println!("wumpus is at {}, {}", wumpus.x+1, wumpus.y+1);
    //println!("cave is at {}, {}", cave.x+1, cave.y+1);
    //println!("bats is at {}, {}", bats.x+1, bats.y+1);
    // println!("player is at {}, {}", player.x+1, player.y+1);


    let mut grid:[[(bool, char); 5]; 5]  = [[(false, 'x'); 5]; 5];
    base_logic(&mut grid, &mut player, wumpus, cave, bats, &mut arrow);
    grid_print(grid);
    player_hint(player, wumpus, bats, cave);
    main_loop(&mut player, &mut wumpus, &mut arrow, bats, cave, & mut grid);
    death_print(player);




}
