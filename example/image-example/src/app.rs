use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_image::{provide_image_context, Image};
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    provide_image_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>
        <Title text="Welcome to Leptos"/>
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx, <ErrorTemplate outside_errors/> }
                .into_view(cx)
        }>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=|cx| {
                            view! { cx,
                                <div
                                    style:display="flex"
                                    style:width="20rem"
                                    style:justify-content="space-between"
                                    style:margin-left="auto"
                                    style:margin-right="auto"
                                >
                                    <div>
                                        <a href="/1">"Example Medium"</a>
                                    </div>
                                    <div>
                                        <a href="/2">"Example Large"</a>
                                    </div>
                                </div>
                                <Outlet/>
                            }
                        }
                    >
                        <Route
                            path="/"
                            view=|cx| {
                                view! { cx, <h1>"Welcome to Leptos Image"</h1> }
                            }
                        />
                        <Route
                            path="/1"
                            view=|cx| {
                                view! { cx, <ImageComparison width=500 height=500/> }
                            }
                        />
                        <Route
                            path="/2"
                            view=|cx| {
                                view! { cx, <ImageComparison width=1000 height=1000/> }
                            }
                        />
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn ImageComparison(cx: Scope, width: u32, height: u32) -> impl IntoView {
    view! { cx,
        <div
            style:margin-left="auto"
            style:margin-right="auto"
            style:display="flex"
            style:justify-content="space-around"
            style:align-items="center"
            style:gap="1rem"
        >
            <div>
                <div>
                    <h1>"Optimized with blur preview"</h1>
                </div>
                <Image
                    src="/cute_ferris.png"
                    width
                    height
                    quality=85
                    blur=true
                    class="test-image"
                />
            </div>
            <div>
                <div>
                    <h1>"Normal Image"</h1>
                </div>
                <img src="/cute_ferris.png" class="test-image"/>
            </div>
        </div>
    }
}
