extern crate futures_timer;

use crate::walkers::{default_walker::DefaultWalker, player::Player, walker::AsyncWalker};
use futures_timer::Delay;
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

pub struct Game {
    player: Arc<RwLock<Player>>,
    enemy1: Arc<RwLock<DefaultWalker>>,
    enemy2: Arc<RwLock<DefaultWalker>>,
}

impl Game {
    #[inline]
    pub fn new(
        player: Arc<RwLock<Player>>,
        enemy1: Arc<RwLock<DefaultWalker>>,
        enemy2: Arc<RwLock<DefaultWalker>>,
    ) -> Self {
        Self {
            player,
            enemy1,
            enemy2,
        }
    }

    #[inline]
    pub async fn start(&mut self) {
        let player = &mut self.player;
        let enemy1 = &mut self.enemy1;
        let enemy2 = &mut self.enemy2;

        let p = async {
            Delay::new(Duration::from_secs(1)).await;
            player.start().await
        };

        let e1 = async {
            Delay::new(Duration::from_secs(1)).await;
            enemy1.start().await
        };

        let e2 = async {
            Delay::new(Duration::from_secs(1)).await;
            enemy2.start().await
        };

        futures::future::join3(p, e1, e2).await;
    }
}
