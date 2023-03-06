#[macro_use(lazy_static)]
extern crate lazy_static;

pub mod routes {
    pub mod hi;
    pub mod user;

    pub mod email {
        pub mod get_email_list;
    }
    pub mod auth {
        pub mod auth;
        pub mod login;
        pub mod register;
    }
}
pub mod helpers {

}    
pub mod middleware {       
    pub mod authentication;
    pub mod authorization;
}

pub mod structs {
    pub mod opt_struct;
    pub mod auth {
        pub mod user_struct;
    }
}

pub mod database {
    pub mod postgres;
    pub mod mongo;
}

pub mod router;
pub mod utils {
    pub mod globals;
}
