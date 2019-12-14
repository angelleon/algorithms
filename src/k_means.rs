extern crate rand;
use rand::Rng;

use utils::{println_num_Vec, sum_usize_vec};

pub use super::data_structures::points::{KPoint, NPoint, Point};

fn sum_kcounts(k_points: &Vec<KPoint>) -> usize {
    let mut sum = 0usize;
    for i in 0..k_points.len() {
        sum += k_points[i].count;
    }
    sum
}

fn compute_means(k_points: &mut Vec<KPoint>, n_points: &Vec<NPoint>) {
    let mut x_sums = vec![0i32; k_points.len()];
    let mut y_sums = vec![0i32; k_points.len()];
    let mut i = 0usize;
    let mut klasif: usize;
    while i < n_points.len() {
        klasif = n_points[i].clasif;
        x_sums[klasif] += n_points[i].p.x;
        y_sums[klasif] += n_points[i].p.y;
        i += 1;
        // 38.65.128.115
    }
    i = 0;
    let mut x_mean: f64;
    let mut y_mean: f64;
    let mut count = 0usize;
    while i < x_sums.len() {
        count = k_points[i].count;
        if count == 0 {
            i += 1;
            continue;
        }
        x_mean = x_sums[i] as f64 / count as f64;
        y_mean = y_sums[i] as f64 / count as f64;
        k_points[i].p.x = x_mean as i32;
        k_points[i].p.y = y_mean as i32;
        i += 1;
    }
}

fn clasif_points(k_points: &mut Vec<KPoint>, n_points: &mut Vec<NPoint>) -> usize {
    let mut min_dist: f64;
    let mut curr_dist: f64;
    let mut curr_clasif: usize;
    let mut new_clasif: usize;
    let mut changes = 0usize;
    for i in 0..n_points.len() {
        // assert_eq!(sum_usize_vec(&clasif_count), n_points.len());
        curr_clasif = n_points[i].clasif;
        new_clasif = curr_clasif;
        min_dist = std::f64::INFINITY;
        for j in 0..k_points.len() {
            //println!("k.p[{}]: {}", j, k_points[j].p.to_string());
            curr_dist = n_points[i].p.dist(&k_points[j].p);
            if curr_dist < min_dist {
                min_dist = curr_dist;
                new_clasif = j;
            }
        }
        if new_clasif != curr_clasif {
            //assert_eq!(sum_kcounts(k_points), n_points.len());
            //if curr_clasif <= k_points.len() {
            k_points[curr_clasif].count -= 1;
            //}
            k_points[new_clasif].count += 1;
            //assert_eq!(sum_kcounts(k_points), n_points.len());
            n_points[i].clasif = new_clasif;
            changes += 1;
        }
    }
    changes
}

fn populate_vectors(k: usize, n: usize, width: i32, height: i32) -> (Vec<KPoint>, Vec<NPoint>) {
    let mut rng = rand::thread_rng();
    let mut k_points = Vec::<KPoint>::new();
    let mut n_points = Vec::<NPoint>::new();
    for _ in 0..k {
        k_points.push(KPoint::new_from(
            Point::new_from(rng.gen_range(0, width), rng.gen_range(0, height)),
            0usize,
        ));
    }
    let index = k_points.len() - 1;
    k_points[index].count = n;
    for _ in 0..n {
        n_points.push(NPoint::new_from(
            Point::new_from(rng.gen_range(0, width), rng.gen_range(0, height)),
            k - 1,
        ));
    }
    (k_points, n_points)
}

pub fn k_means(
    n: usize,
    k: usize,
    width: i32,
    height: i32, percent: f64,
) -> (Vec<KPoint>, Vec<NPoint>) {
    let (mut k_points, mut n_points) = populate_vectors(k, n, width, height);
    let mut changes = clasif_points(&mut k_points, &mut n_points);
    let mut i = 0;
    loop {
        compute_means(&mut k_points, &n_points);
        changes = clasif_points(&mut k_points, &mut n_points);
        if (changes as f64 / n as f64) < percent || i >= k_points.len() {
            break;
        }
        i += 1;
    }
    println!("cahnges: {}", changes);
    println!("iterations: {}", i);
    (k_points, n_points)
}
