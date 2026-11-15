# 05 — Concurrency and async

채널, actor, bounded queue, async pipeline에서 소유권과 종료 규칙을 공부합니다.

## 패턴 목록

| 주제 | 강의 코드 | 연습 |
| --- | --- | --- |
| Channel pipeline | [직접 구현 / 적용](01-channel-pipeline/) | [exercise](../../exercise/05-concurrency-async/01-channel-pipeline/) |
| Actor | [직접 구현 / 적용](02-actor/) | [exercise](../../exercise/05-concurrency-async/02-actor/) |
| Work queue | [직접 구현 / 적용](03-work-queue/) | [exercise](../../exercise/05-concurrency-async/03-work-queue/) |
| Async pipeline | [직접 구현 / 적용](04-async-pipeline/) | [exercise](../../exercise/05-concurrency-async/04-async-pipeline/) |

공유 Job 타입은 `00-core/`에 있고, 챕터 루트의 `src/lib.rs`는 호환용 facade입니다.

## 실행

```sh
cargo +stable test -p concurrency-async
```

