use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let count_times_two = move || count.get() * 2;

    view! { cx,
        <button
            on:click=move |_| {
                set_count.set(count.get() + 1);
            }
        >
            "Click me!"
        </button>
        <button on:click=move |_| { set_count.set(0) }>"Reset"</button>
        <p class=("red", move || count.get() % 2 == 1)>Current count: {move || count.get()}</p>
        <p>Count * 2: {count_times_two}</p>
    }
}
