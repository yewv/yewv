mod common;

use common::*;
use wasm_bindgen_test::wasm_bindgen_test;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

struct TestContext {
    props: StoreAppProps,
}

fn setup() -> TestContext {
    TestContext {
        props: StoreAppProps::new(SubscriptionType::Watch),
    }
}

#[wasm_bindgen_test]
async fn on_store_value_changed_with_new_value_should_rerender() {
    //Given
    let ctx = setup();
    render_with_props::<StoreApp>(ctx.props.clone()).await;
    let render_count = *ctx.props.render_count.borrow();
    //When
    ctx.props.context.set_state(StoreState { value: 1 });
    //Then
    wait().await;
    assert_eq!(*ctx.props.render_count.borrow(), render_count + 1);
}

#[wasm_bindgen_test]
async fn on_store_value_changed_with_trivial_value_change_should_not_rerender() {
    //Given
    let ctx = setup();
    render_with_props::<StoreApp>(ctx.props.clone()).await;
    let render_count = *ctx.props.render_count.borrow();
    //When
    ctx.props.context.set_state(StoreState { value: 0 });
    //Then
    wait().await;
    assert_eq!(*ctx.props.render_count.borrow(), render_count);
}
