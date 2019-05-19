use futures::Future;
use oxford::Client;

#[test]
fn get_word() {
    tokio::run(
        Client::new("abc".to_string(), "def".to_string(), "en-ca".to_string())
            .lookup_word("work")
            .and_then(|res| {
                println!("{:?}", res);
                assert_eq!(false, true);
                Ok(res)
            }),
    );
}
