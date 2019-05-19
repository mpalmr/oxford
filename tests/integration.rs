use oxford::Client;
use futures::Future;

#[test]
fn get_word() -> impl Future<Item = (), Error = ()> {
    Client::new("abc".to_string(), "def".to_string(), "en-ca".to_string())
        .lookup_word("work")
        .and_then(|res| {
            println!("{:?}", res);
        })
}
