use structural::{
    bridge::{Brackets, Notice, Plain},
    flyweight::StylePool,
};
#[path = "../02-bridge/examples/02_bridge.rs"]
mod after_bridge;
#[path = "../06-flyweight/examples/02_flyweight.rs"]
mod after_flyweight;
#[path = "../02-bridge/examples/01_direct.rs"]
mod before_bridge;
#[path = "../06-flyweight/examples/01_direct.rs"]
mod before_flyweight;
#[test]
fn bridge_varies_message_policy_and_renderer_independently() {
    for text in ["", "hello"] {
        for urgent in [false, true] {
            for brackets in [false, true] {
                assert_eq!(
                    before_bridge::render(text, urgent, brackets),
                    after_bridge::render(text, urgent, brackets)
                );
            }
        }
    }
    assert_eq!(Notice::new(Plain).render("hi", true), "! hi");
    assert_eq!(Notice::new(Brackets).render("hi", false), "[hi]");
}
#[test]
fn flyweight_shares_intrinsic_style_but_not_position() {
    let mut pool = StylePool::default();
    let a = pool.glyph("red", 1);
    let b = pool.glyph("red", 2);
    let c = pool.glyph("blue", 1);
    assert!(std::rc::Rc::ptr_eq(&a.style, &b.style));
    assert!(!std::rc::Rc::ptr_eq(&a.style, &c.style));
    assert_eq!(a.render(), "red@1");
    assert_eq!(b.render(), "red@2");
    for items in [vec![], vec![("red", 1), ("red", 2), ("blue", 1)]] {
        assert_eq!(
            before_flyweight::render(&items),
            after_flyweight::render(&items)
        );
    }
}
