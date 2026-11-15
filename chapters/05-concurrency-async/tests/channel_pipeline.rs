// Coverage boundary: these fixtures do not force scrambled completion order.
// Sorting is exercised under ordinary scheduling only. Worker-panic/join failure
// injection is not verified; see docs/course/verification-boundaries.md.
mod support;
use concurrency_async::{
    channel_pipeline::{run_pipeline, PipelineError},
    Job, JobResult,
};

#[test]
fn input_order_is_preserved_even_with_duplicate_unordered_ids() {
    support::bounded(|| {
        for workers in [1, 2, 8] {
            assert_eq!(
                run_pipeline(
                    vec![
                        Job { id: 9, value: 3 },
                        Job { id: 2, value: 0 },
                        Job {
                            id: 9,
                            value: u32::MAX
                        },
                    ],
                    workers
                ),
                Ok(vec![
                    JobResult { id: 9, value: 9 },
                    JobResult { id: 2, value: 0 },
                    JobResult {
                        id: 9,
                        value: 18_446_744_065_119_617_025
                    },
                ])
            );
        }
    });
}

#[test]
fn zero_workers_is_rejected_including_empty_input() {
    assert_eq!(run_pipeline(vec![], 0), Err(PipelineError::ZeroWorkers));
    assert_eq!(
        run_pipeline(vec![Job { id: 1, value: 2 }], 0),
        Err(PipelineError::ZeroWorkers)
    );
}

#[test]
fn finite_input_closes_channels_and_joins_workers() {
    support::bounded(|| {
        assert_eq!(run_pipeline(vec![], 3), Ok(vec![]));
        let jobs = (0..200).map(|id| Job { id, value: 2 }).collect();
        let output = run_pipeline(jobs, 2).unwrap();
        assert_eq!(output.len(), 200);
        for (id, result) in output.iter().enumerate() {
            assert_eq!(
                *result,
                JobResult {
                    id: id as u64,
                    value: 4
                }
            );
        }
    });
}
