extern crate futures;
extern crate futures_timer;
extern crate rand;

use futures::executor::block_on;
use futures_timer::Delay;
use moving_game::{
    coordinates::{coordinate::Coordinate, default_coordinate::DefaultCoordinate},
    game::Game,
    walkers::{default_walker::DefaultWalker, player::Player, walker::Walker},
};
use rand::Rng;
use std::{
    process::exit,
    sync::{Arc, RwLock},
    time::Duration,
};

#[inline]
fn exit_if_game_is_finished(
    e1: &impl Coordinate,
    e2: &impl Coordinate,
    player: Arc<RwLock<Player>>,
) {
    let (coordinate, is_finished) = {
        let p = (*player).read().unwrap();
        (p.get_coordinate().clone(), p.is_finished())
    };

    if e1.equals(&coordinate) || e2.equals(&coordinate) {
        println!("Player has lost");
        exit(0)
    } else if is_finished {
        println!("Player has won");
        exit(0)
    }
}

#[inline]
async fn draw_table_while_playing(
    max_x: u32,
    max_y: u32,
    player: Arc<RwLock<Player>>,
    enemy1: Arc<RwLock<impl Walker>>,
    enemy2: Arc<RwLock<impl Walker>>,
) {
    loop {
        let p = {
            let p = (*player).read().unwrap();
            p.get_coordinate().clone()
        };

        let e1 = {
            let e1 = (*enemy1).read().unwrap();
            e1.get_coordinate().clone()
        };

        let e2 = {
            let e2 = (*enemy2).read().unwrap();
            e2.get_coordinate().clone()
        };

        (0..max_y).for_each(|i| {
            (0..max_x).for_each(|q| {
                let cur_coordinate = DefaultCoordinate::new(q, i);

                if cur_coordinate.equals(&p) {
                    print!("P");
                }

                if cur_coordinate.equals(&e1) {
                    print!("1");
                }

                if cur_coordinate.equals(&e2) {
                    print!("2");
                }

                print!(
                    "{}",
                    if !cur_coordinate.equals(&p)
                        && !cur_coordinate.equals(&e1)
                        && !cur_coordinate.equals(&e2)
                    {
                        "x "
                    } else {
                        " "
                    }
                );
            });

            println!();
        });

        println!();

        exit_if_game_is_finished(&e1, &e2, player.clone());

        Delay::new(Duration::from_secs(1)).await;
    }
}

#[inline]
fn read_u32() -> u32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

#[inline]
async fn start(
    mut game: Game,
    columns: u32,
    rows: u32,
    player: Arc<RwLock<Player>>,
    enemy1: Arc<RwLock<impl Walker>>,
    enemy2: Arc<RwLock<impl Walker>>,
) {
    let g = game.start();
    let d = draw_table_while_playing(columns, rows, player, enemy1, enemy2);
    futures::future::join(g, d).await;
}

fn main() {
    println!("Amount of rows:");
    let rows = read_u32();

    println!("Amount of columns:");
    let columns = read_u32();

    let player = Arc::new(RwLock::new(Player::new(0, 0, columns - 1, rows - 1)));

    let enemy1 = Arc::new(RwLock::new(DefaultWalker::new(
        rand::thread_rng().gen_range(0..columns),
        rand::thread_rng().gen_range(0..rows),
        columns - 1,
        rows - 1,
    )));

    let enemy2 = Arc::new(RwLock::new(DefaultWalker::new(
        rand::thread_rng().gen_range(0..columns),
        rand::thread_rng().gen_range(0..rows),
        columns - 1,
        rows - 1,
    )));

    let game = Game::new(player.clone(), enemy1.clone(), enemy2.clone());

    block_on(start(
        game,
        columns,
        rows,
        player.clone(),
        enemy1.clone(),
        enemy2.clone(),
    ))
}
