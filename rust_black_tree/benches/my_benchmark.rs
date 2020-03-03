use std::iter;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use rust_black_trees::tree::Tree;
use rust_black_trees::{avltree::AVLTree, rbtree::RBTree, unbalancetree::BSTree};

fn bench_rbtree(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_rbtree");

    let mut rbtree = RBTree::new();
    for tree_size in [10000, 40000, 70000, 100000, 130000].iter() {
        //        group.throughput(Throughput::Bytes(*tree_size));
        group.bench_with_input(
            BenchmarkId::from_parameter(tree_size),
            tree_size,
            |b, &tree_size| b.iter(|| insert_n_elements_and_search_lowest(&mut rbtree, tree_size)),
        );
    }
    group.finish();
}

fn bench_avltree(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_avltree");

    let mut avltree = AVLTree::new();
    for tree_size in [10000, 40000, 70000, 100000, 130000].iter() {
        //        group.throughput(Throughput::Bytes(*tree_size));
        group.bench_with_input(
            BenchmarkId::from_parameter(tree_size),
            tree_size,
            |b, &tree_size| b.iter(|| insert_n_elements_and_search_lowest(&mut avltree, tree_size)),
        );
    }
    group.finish();
}

fn bench_bstree(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_bstree");

    let mut bstree = BSTree::new();
    for tree_size in [10000, 40000, 70000, 100000, 130000].iter() {
        //        group.throughput(Throughput::Bytes(*tree_size));
        group.bench_with_input(
            BenchmarkId::from_parameter(tree_size),
            tree_size,
            |b, &tree_size| b.iter(|| insert_n_elements_and_search_lowest(&mut bstree, tree_size)),
        );
    }
    group.finish();
}
criterion_group!(benches, bench_rbtree, bench_avltree, bench_bstree);
criterion_main!(benches);

fn insert_n_elements_and_search_lowest<T>(tree: &mut T, num_times: usize)
where
    T: Tree<usize>,
{
    for i in 0..num_times {
        tree.insert(i);
    }

    for i in 0..num_times / 10 {
        tree.contains(&i);
    }
}
