use creational::{abstract_factory as family, object_pool::BufferPool};
#[path = "../02-abstract-factory/examples/02_abstract_factory.rs"]
mod after_factory;
#[path = "../05-object-pool/examples/02_object_pool.rs"]
mod after_pool;
#[path = "../02-abstract-factory/examples/01_direct.rs"]
mod before_factory;
#[path = "../05-object-pool/examples/01_direct.rs"]
mod before_pool;
#[test]
fn abstract_factory_keeps_both_products_in_the_selected_family() {
    for (dark, expected) in [
        (false, ("light button".into(), "light checkbox".into())),
        (true, ("dark button".into(), "dark checkbox".into())),
    ] {
        assert_eq!(before_factory::screen(dark), expected);
        assert_eq!(after_factory::screen(dark), expected);
    }
    assert_eq!(
        family::screen(&family::Dark),
        ("dark button".into(), "dark checkbox".into())
    );
}
#[test]
fn pool_is_bounded_clears_and_returns_buffers_even_during_unwind() {
    let pool = BufferPool::new(1);
    let mut first = pool.acquire().unwrap();
    first.extend_from_slice(b"secret");
    let allocated = first.capacity();
    assert!(pool.acquire().is_none());
    drop(first);
    let reused = pool.acquire().unwrap();
    assert!(reused.is_empty());
    assert!(reused.capacity() >= allocated);
    drop(reused);
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut lease = pool.acquire().unwrap();
        lease.push(1);
        panic!("release lease");
    }));
    assert!(pool.acquire().unwrap().is_empty());
    assert!(BufferPool::new(0).acquire().is_none());
    for input in [vec![], vec!["", "hello", "rust"]] {
        assert_eq!(before_pool::lengths(&input), after_pool::lengths(&input));
    }
    assert_eq!(after_pool::lengths(&["", "hi"]), [0, 2]);
}
