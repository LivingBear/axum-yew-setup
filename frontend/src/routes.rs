
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    LoginPage,
    #[at("/register")]
    RegisterData,
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
        Route::RegisterData => html! {<RegisterData />  },
        Route::ContactPage => html! {<ContactPage />  },
        Route::EmailPage => html! {<EmailPage />  },
        Route::AdminPage => html! {<AdminPage />  },
        Route::UsersPage => html! {<UsersPage />  }
    }
}
