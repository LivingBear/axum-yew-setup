use async_smtp::{EmailAddress, Envelope, SendableEmail, SmtpClient};

pub fn start_smtp_server() {
        async_std::task::block_on(async move {
        let email = SendableEmail::new(
            Envelope::new(
                Some(EmailAddress::new("gcppdan@gmail.com".to_string()).unwrap()),
                vec![EmailAddress::new("dan@gcpp.gold".to_string()).unwrap()],
            )
            .unwrap(),
            "id",
            "Hello ß☺ example".to_string().into_bytes(),
        );

        // Open a local connection on port 25
        let mut mailer = SmtpClient::new_unencrypted_localhost().into_transport();
        // Send the email
        let result = mailer.connect_and_send(email).await;

        if result.is_ok() {
            println!("Email sent");
        } else {
            println!("Could not send email: {:?}", result);
        }

        assert!(result.is_ok());
    });
}