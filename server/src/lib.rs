pub mod routes {
    pub mod hi;
    pub mod email {
        pub mod get_email_list;
    }
    pub mod auth {
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
pub mod services {
    pub mod email {
        pub mod smtp;
    }
    pub mod auth {}
    pub mod orders {}
    pub mod products {}
    pub mod services {}
}

pub mod database;
pub mod router;

