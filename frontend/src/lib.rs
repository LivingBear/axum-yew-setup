pub mod components {
    pub mod head;
    pub mod header;
    pub mod tailwind_header;
    pub mod footer;
    pub mod requests {
        pub mod hi;
        pub mod email_data;
    }
    pub mod auth {
        pub mod login;
        pub mod register;
    }
    pub mod generic {
        pub mod loading;
        pub mod error {
            pub mod no_response;
            pub mod some_error_response;
        }
        pub mod success {
            pub mod some_response;
        }
    }
    //other components
}

pub mod pages {
    pub mod public {
        pub mod login;
        pub mod registration;
        pub mod home;
        pub mod about;
        pub mod email;
        pub mod services;
        pub mod products;
        pub mod contact;
    }
    pub mod private {
        pub mod profile;
        pub mod admin;
        pub mod users;
    }
}

pub mod models { 
    pub mod components {
        pub mod server_request;
    }
}

pub mod handlers {
}