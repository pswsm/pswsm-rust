#[component]
pub fn Navbar<T: IntoView>(
    cx: Scope,
    title: String,
    #[prop(optional)] elements: Vec<T>,
) -> impl IntoView {
    view! { cx,
        <nav class="nav">
            <p>{title}</p>
            {
                elements.into_iter().map(|e| view! { cx, <div>{e}</div>}).collect::<Vec<_>>()
            }
        </nav>
    }
}
