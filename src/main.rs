#![allow(unused)]

extern crate rand;
extern crate xcg;
extern crate console;
extern crate crossbeam;
extern crate priority_queue;

use rand::IsaacRng;
use rand::prelude::RngCore;
use std::cell::RefCell;

use xcg::model::*;
use xcg::bot::Bot2;
use std::thread;
use std::time::Duration;

use xcg::utils::Trim;
use xcg::model::*;
use xcg::bot::common::{P, a_star_find};
use xcg::bot::common::distance;
use priority_queue::PriorityQueue;
use xcg::bot::common::W;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

fn main() {
    let mut gs = game_state(r#"
        *.*.*.*.*.*.*.*.*.*.*.
        *. . . . . . . . . .*.
        *. . . . . . . . . .*.
        *. . . . a a a . . .*.
        *. . . . a A a . . .*.
        *. . . . . . a . . .*.
        *. . a a a a a . . .*.
        *. . . . . . . . . .*.
        *.*.*.*.*.*.*.*.*.*.*.
    "#);
    // decartes coordinates
    let src = P(5, 4);
    let m = gs.field.m as i16;
    let n = gs.field.n as i16;
    let me = gs.players[0].body().iter().map(|p| P(p.1, m - 1 - p.0)).collect::<Vec<P>>();
    let is_boundary = |p: &P| {
        let P(x, y) = *p;
        0 <= y && y < m && 0 <= x && x < n && !me.contains(&p)
    };
    let heuristic = |p: &P, q: &P| distance(p, q);
    let mut logger = |ol: &PriorityQueue<P, W>, cl: &HashSet<P>| {
        for (k, _) in ol {
            let P(x, y) = *k;
            let j = x as usize;
            let i = (m as usize) - 1 - (y as usize);
            gs.field.cells[i][j] = Cell::Owned(0);
        }
        for p in cl {
            let P(x, y) = *p;
            let j = x as usize;
            let i = (m as usize) - 1 - (y as usize);
            gs.field.cells[i][j] = Cell::Owned(1);
        }
        println!("{}", prettify_game_state(&gs, false, true));
        println!("{:?}", ol);
    };

    let dst = P(9, 2);
    let path = a_star_find(&src, &dst, is_boundary, heuristic, logger);
    println!("{:?}", path);
}

fn game_state(gs: &str) -> GameState {
    GameState::parse_string(&gs.trim_indent()).unwrap()
}

fn main1() {
    let random = RefCell::new(IsaacRng::new_from_u64(234));
    let m = 32;
    let n = 54;
    let timeout = 30;

    let a = Bot2::new(0);
    let b = Bot2::new(1);
    let c = Bot2::new(2);
    let d = Bot2::new(3);
//    let mut bots: [Box<dyn Bot>; 1] = [Box::new(d)];
    let mut bots: [Box<dyn Bot>; 4] = [Box::new(a), Box::new(b), Box::new(c), Box::new(d)];
    let names: Vec<String> = bots.iter().enumerate()
        .map(|(k, _)| ((('A' as u8) + (k as u8)) as char).to_string())
        .collect();

    let logger = |gs: &GameState| {
        if gs.stats.iteration > 0 {
            println!("{}", prettify_game_state(gs, true, true));
            thread::sleep(Duration::from_millis(timeout));
        }
    };

    let count = 1_000_000;
    let random = RefCell::new(IsaacRng::new_from_u64(234));
    let mut seeds = Vec::with_capacity(count);
    for it in 0..count {
        let match_k_seed = random.borrow_mut().next_u64();
        seeds.push(match_k_seed);
    }

    for it in 0..100 {
//        let match_k_seed = random.borrow_mut().next_u64();
//        let match_k_seed = 2216562425439805338;
        let match_k_seed = seeds[it];
        let mut match_k = create_match(m, n, &names, 1024, 0.9, Some(match_k_seed));
        let _replay_k = run_match(&mut match_k, &mut bots, &logger);
//        println!("{} {:?}", "\n".repeat(m + names.len()), match_k.game_state.stats);
        let i = match_k.game_state.stats.iteration;
        let o = match_k.game_state.stats.ouroboros_count;
        let b = match_k.game_state.stats.bite_count;
        let h = match_k.game_state.stats.head_to_head_count;
        let s = match_k.game_state.stats.scores;
        println!("{:06}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", it, i, o, b, h, match_k_seed, s[0], s[1], s[2], s[3]);
    }
}
