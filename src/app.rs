use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/login-test.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/users/:name" view=UserProfile/>
                    <Route path="/login" view=LoginPage/>
                    <Route path="/signup" view=SignUpPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
fn UserProfile() -> impl IntoView {
    let params = use_params_map();
    let name = move || params.with(|params| params.get("name").cloned().unwrap_or_default());

    view! {
        <h1> Hi {name}</h1>
    }
}


#[server(LoginUser)]
pub async fn login_user(username: String, password: String) -> Result<(), ServerFnError> {
    println!("{}, {}", username, password);
    todo!();
    return Ok(());
}

#[component]
fn LoginPage() -> impl IntoView {
    let login_user = create_server_action::<LoginUser>();
    // holds the latest *returned* value from the server
    let value = login_user.value();
    // check if the server has returned an error
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    view! {
        <h1> Login in </h1>
        <h2> Please enter your <del>highly profitable</del> awesome information :3 </h2>
        <ActionForm action=login_user>
            <label>
                "Username: "
                // `title` matches the `title` argument to `add_todo`
                <input type="text" name="username"/>
            </label>
            <label>
                "Password: "
                <input type="password" name="password"/>
            </label>
            <input type="submit" value="Submit!"/>
        </ActionForm>
    }
}



#[server(SignUpUser)]
pub async fn sign_up_user(username: String, password: String) -> Result<(), ServerFnError> {
    println!("{}, {}", username, password);

    use sqlx::sqlite::SqliteConnection;

    let connection = SqliteConnection::connect("sqlite:/tmp/test").await?;
    todo!();
    return Ok(());
}


#[component]
fn SignUpPage() -> impl IntoView {
    let sign_up_user = create_server_action::<SignUpUser>();
    // holds the latest *returned* value from the server
    let value = sign_up_user.value();
    // check if the server has returned an error
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    view! {
        <h1> Sign Up </h1>
        <h2> To let us <del>harvest</del> admire your epic data :3 </h2>
        <ActionForm action=sign_up_user>
            <label>
                "Username: "
                // `title` matches the `title` argument to `add_todo`
                <input type="text" name="username"/>
            </label>
            <label>
                "Password: "
                <input type="password" name="password"/>
            </label>
            <input type="submit" value="Submit!"/>
        </ActionForm>
    }
}