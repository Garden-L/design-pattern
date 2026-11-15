use creational::object_pool::BufferPool;
pub fn lengths(input: &[&str]) -> Vec<usize> {
    let pool = BufferPool::new(1);
    input
        .iter()
        .map(|s| {
            let mut buffer = pool
                .acquire()
                .expect("previous iteration returned its lease");
            buffer.extend_from_slice(s.as_bytes());
            buffer.len()
        })
        .collect()
}
