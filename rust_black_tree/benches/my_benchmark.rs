use std::iter;

use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;

fn bench_rbtree(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_rbtree");

    let rbtree = RbTree::new();
    for tree_size in [10000, 40000, 70000, 100000, 130000].iter() {
        //        group.throughput(Throughput::Bytes(*tree_size));
        group.bench_with_input(
            BenchmarkId::from_parameter(tree_size),
            tree_size,
            |b, &tree_size| insert_n_elements_and_search_lowest(rbtree, tree_size),
        );
    }
    group.finish();
}

criterion_group!(benches, from_elem);
criterion_main!(benches);
/*
        for tree_size in &[10000, 40000, 70000, 100000, 130000] {
            print!("Time for Red Black Tree, {} elements: ", tree_size);
            std::io::stdout().flush().unwrap();
            let rbtree = rbtree::Tree::new();
            let now = Instant::now();
            insert_n_elements_and_search_lowest(rbtree, tree_size);
            println!("{} milliseconds", now.elapsed().as_millis());
            println!();

            print!("Time for AVL Tree, {} elements: ", tree_size);
            std::io::stdout().flush().unwrap();
            let avltree = rbtree::Tree::new();
            let now = Instant::now();
            insert_n_elements_and_search_lowest(avltree, tree_size);
            println!("{} milliseconds", now.elapsed().as_millis());
            println!();
        }
    }

*/
fn insert_n_elements_and_search_lowest<T>(tree: T, num_times: usize)
//where T: // tree trait
{
    for i in 0..num_times {
        tree.insert(i);
    }

    for i in 0..num_times / 10 {
        tree.contains(i);
    }
}
