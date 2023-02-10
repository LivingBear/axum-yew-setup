use frontend::components::tailwind_header::TailwindHeaderComponent;
use frontend::pages::private::users::UsersPage;
use yew::prelude::*;
use yew_router::prelude::*;
use frontend::pages::public::contact::ContactPage;
use frontend::pages::public::login::LoginPage;
use frontend::pages::public::home::HomePage;
use frontend::pages::public::about::AboutPage;
use frontend::pages::public::registration::RegistrationPage;
use frontend::pages::public::products::ProductsPage;
use frontend::pages::public::services::ServicesPage;
use frontend::pages::public::email::EmailPage;
use frontend::pages::private::admin::AdminPage;
use frontend::components::requests::hi::HiServer;
use frontend::components::requests::email_data::EmailData;
// use frontend::components::header::HeaderComponent;
use frontend::components::footer::FooterComponent;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    LoginPage,
    #[at("/register")]
    RegistrationPage,
    #[at("/products")]
    ProductsPage,
    #[at("/services")]
    ServicesPage,
    #[at("/about")]
    AboutPage,
    #[at("/contact")]
    ContactPage,
    #[at("/email")]
    EmailPage,
    #[at("/hi-server")]
    HiServer,
    #[at("/get_email_list")]
    EmailData,
    #[at("/admin")]
    AdminPage,
    #[at("/users")]
    UsersPage

}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage/> },
        Route::HiServer => html! { <HiServer /> },
        Route::EmailData => html! { <EmailData /> },
        Route::LoginPage => html! {<LoginPage />  },
        Route::AboutPage => html! {<AboutPage />  },
        Route::ProductsPage => html! {<ProductsPage />  },
        Route::ServicesPage => html! {<ServicesPage />  },
        Route::RegistrationPage => html! {<RegistrationPage />  },
        Route::ContactPage => html! {<ContactPage />  },
        Route::EmailPage => html! {<EmailPage />  },
        Route::AdminPage => html! {<AdminPage />  },
        Route::UsersPage => html! {<UsersPage />  }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        
        <>
        <section class="wrapper">
            <TailwindHeaderComponent />
                <main>
                    <BrowserRouter>
                        <Switch<Route> render={switch} />
                    </BrowserRouter>
                </main>
            <FooterComponent />
        </section>
        </>
    }
}


fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
