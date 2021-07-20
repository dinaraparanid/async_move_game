extern crate async_trait;
extern crate futures;
extern crate futures_timer;
extern crate rand;

use crate::coordinates::{coordinate::Coordinate, mutable_coordinate::MutableCoordinate};
use async_trait::async_trait;
use futures_timer::Delay;
use rand::Rng;
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

pub trait Walker {
    fn get_max_x(&self) -> u32;
    fn get_max_y(&self) -> u32;
    fn get_coordinate(&self) -> &MutableCoordinate;
    fn get_coordinate_mut(&mut self) -> &mut MutableCoordinate;
    fn new(x: u32, y: u32, max_x: u32, max_y: u32) -> Self
    where
        Self: Sized;
}

#[async_trait]
pub(crate) trait AsyncWalker {
    async fn move_async(&self);
    async fn start(&self);
}

#[async_trait]
impl<T: Walker + Send + Sync> AsyncWalker for Arc<RwLock<T>> {
    #[inline]
    async fn move_async(&self) {
        loop {
            match rand::thread_rng().gen_range(0..4) {
                0 => {
                    let y = {
                        let r = (**self).read().unwrap();
                        let r = r.get_coordinate().clone();
                        r.get_y()
                    };

                    if y > 0 {
                        let mut w = (**self).write().unwrap();
                        w.get_coordinate_mut().up();
                        break;
                    }
                }

                1 => {
                    let (y, max_y) = {
                        let r = (**self).read().unwrap();
                        let r2 = r.get_coordinate().clone();
                        (r2.get_y(), r.get_max_y())
                    };

                    if y < max_y {
                        let mut w = (**self).write().unwrap();
                        w.get_coordinate_mut().down();
                        break;
                    }
                }

                2 => {
                    let (x, max_x) = {
                        let r = (**self).read().unwrap();
                        let r2 = r.get_coordinate().clone();
                        (r2.get_x(), r.get_max_y())
                    };

                    if x < max_x {
                        let mut w = (**self).write().unwrap();
                        w.get_coordinate_mut().right();
                        break;
                    }
                }

                _ => {
                    let x = {
                        let r = (**self).read().unwrap();
                        let r = r.get_coordinate().clone();
                        r.get_x()
                    };

                    if x > 0 {
                        let mut w = (**self).write().unwrap();
                        w.get_coordinate_mut().left();
                        break;
                    }
                }
            }
        }

        Delay::new(Duration::from_secs(2)).await;
    }

    #[inline]
    async fn start(&self) {
        loop {
            self.move_async().await
        }
    }
}
