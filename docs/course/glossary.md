# 용어집

- **Trait object**: 런타임에 구현을 선택하는 `dyn Trait` 값.
- **Static dispatch**: 제네릭을 구체화해 컴파일 시 호출 대상을 결정하는 방식.
- **Typestate**: 타입 파라미터로 유효한 상태와 전이를 표현하는 기법.
- **Newtype**: 기존 타입을 감싸 의미와 타입 안전성을 추가하는 tuple struct.
- **Interior mutability**: 외부에는 불변 참조를 제공하면서 내부 규칙으로 가변성을 관리하는 방식.
- **RAII**: 값의 scope와 `Drop`을 이용해 자원을 정리하는 방식.
- **Backpressure**: 생산 속도가 소비 속도를 넘을 때 bounded queue로 흐름을 제한하는 방식.
- **Cancellation**: 진행 중인 작업을 중단하고 입력·작업·worker를 정리하는 계약.
- **Memento**: 캡슐화를 깨지 않고 상태를 저장·복원하는 snapshot.
- **Flyweight**: 공유 가능한 불변 상태와 요청별 상태를 분리하는 객체.

