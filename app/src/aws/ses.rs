use crate::AppResult;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};
use aws_sdk_sesv2::Client;

#[derive(Clone, Debug)]
pub struct Adapter {
    client: Client,
}

impl Adapter {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn send(&self, to: &str, subject: &str, message: &str) -> AppResult<()> {
        let dest = Destination::builder().to_addresses(to).build();
        let subject_content = Content::builder().data(subject).charset("UTF-8").build();
        let body_content = Content::builder().data(message).charset("UTF-8").build();
        let body = Body::builder().text(body_content).build();

        let msg = Message::builder()
            .subject(subject_content)
            .body(body)
            .build();

        let email_content = EmailContent::builder().simple(msg).build();

        self.client
            .send_email()
            .from_email_address("canvas@no-reply.com")
            .destination(dest)
            .content(email_content)
            .send()
            .await?;

        Ok(())
    }
}
