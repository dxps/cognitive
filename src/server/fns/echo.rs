use dioxus::prelude::*;

// Server functions let us define public APIs on the server that can be called like a normal async function from the client.
// Each server function needs to be annotated with the `#[post]`/`#[get]` attributes, accept and return serializable types, and return
// a `Result` with the error type [`ServerFnError`].
//
// When the server function is called from the client, it will just serialize the arguments, call the API, and deserialize the
// response.
#[post("/api/echo")]
pub async fn echo_server(input: String) -> Result<String> {
    // The body of server function like this comment are only included on the server. If you have any server-only logic like
    // database queries, you can put it here. Any imports for the server function should either be imported inside the function
    // or imported under a `#[cfg(feature = "server")]` block.

    use crate::server::DB;

    // Example of using a database connection inside a server function.
    let db = DB.get();
    info!("[echo] DB active conns: {}", db.size());

    Ok(input)
}
