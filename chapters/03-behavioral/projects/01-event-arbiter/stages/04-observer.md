# 04 — Observer: 변경 통보

중재기는 상태를 바꾼 뒤 변경 결과를 Listener에 전달한다. 필터와 scorer는
Listener를 호출하지 않는다.

```rust
pub trait EventListener {
    fn on_selection_changed(&mut self, result: &SelectionResult);
}
```

실습: UI·사운드·로그를 각각 Listener로 표현하고, 같은 이벤트가 유지될 때
중복 통보가 발생하지 않는지 검증한다.
