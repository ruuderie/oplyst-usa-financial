use anyhow::Result;
use serde_json::{Value, json};


#[tokio::test]
async fn quick_dev() -> Result<(), anyhow::Error> {
    // Create a new httpc test client with a base URL (will be prefixed for all calls)
    // The client will have a cookie_store.
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // do_get, do_post, do_put, do_patch, do_delete return a httpc_test::Response

    // Simple do_get
    let res = hc.do_get("/hello?name=Ruud").await?; // httpc_test::Response 
    let status = res.status();
    println!("Status: {}", status);
    // Pretty print the result (status, headers, response cookies, client cookies, body)
    res.print().await?;

    //hc.do_get("/src/main.rs").await?.print().await?;
    
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "password": "admin"
        })
    );

    //req_login.await?.print().await?;
    
    let create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket 1",
            "description": "Ticket 1 description"
        })
    );
    create_ticket.await?.print().await?;
    let req_tickets = hc.do_get("/api/tickets");
    req_tickets.await?.print().await?;
    //hc.do_delete("/api/tickets/1").await?.print().await?;
    //hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}

