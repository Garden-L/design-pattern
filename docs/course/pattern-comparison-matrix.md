# 패턴 비교표

| 문제 | 우선 검토할 패턴 | Rust 표현 | 적용하지 않을 때 |
| --- | --- | --- | --- |
| 생성 제품군 일관성 | Abstract Factory | trait factory + 제품 계약 | 호출부에 조합 분기 확산 |
| 비싼 객체 재사용 | Object Pool | 소유권 이동 + bounded pool | 생성 비용과 자원 제한을 숨김 |
| 두 축의 독립적 변화 | Bridge | 조합 + generic/trait | 조합별 타입 폭발 |
| 반복 불변 객체 공유 | Flyweight | Arc/HashMap intern | 메모리 중복 증가 |
| 고정 알고리즘의 일부 변경 | Template Method | trait default method | 단계 순서가 구현마다 복제 |
| 컴포넌트 상호작용 | Mediator | message enum + coordinator | 직접 참조 그래프 증가 |
| undo/checkpoint | Memento | private snapshot 타입 | 내부 표현 노출 |
| 작은 DSL | Interpreter | enum AST + evaluator | 문자열 분기 누적 |
| 런타임 정책 교체 | Strategy | trait/generic | 조건문 증가 |
| 유한 상태 전이 | State | enum/typestate | 잘못된 전이 런타임 누락 |
| 작업 생명주기 | Actor/Work Queue | channel + owner + join | shutdown 불명확 |

각 행은 해당 챕터의 적용 전 코드, 적용 후 코드, 동일 계약 테스트와 함께 읽는다.

