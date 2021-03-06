#[macro_use] extern crate json;
#[macro_use] extern crate noir;
#[macro_use]
mod base_test;
test!();


// Multiple Errors ------------------------------------------------------------
#[test]
fn test_multiple_errors_raw_body() {

    let actual = {
        API::get("/status/404")
            .expected_status(StatusCode::Ok)
            .expected_header(Server("Foo".to_string()))
            .expected_body("Hello World")
            .collect()
    };

    assert_fail!(r#"
<br>Response Failure: <bn>GET <by>request to \"<bn>http://localhost:4000<bn>/status/404\" <by>returned <br>3 <by>error(s)

<bb> 1) <by>Response <by>status code does not match value, expected:

        \"<bg>200 OK\"

    <by>but got:

        \"<br>404 Not Found\"

<bb> 2) <by>Response <by>header \"<bb>Server\" <by>was expected <bg>to be present<by>, but <br>is missing<by>.

<bb> 3) <by>Response <by>raw body data does not match, expected the following <bg>11 bytes<by>:

       [<bg>0x48, <bg>0x65, <bg>0x6C, <bg>0x6C, <bg>0x6F, <bg>0x20, <bg>0x57, <bg>0x6F, <bg>0x72, <bg>0x6C, <bg>0x64]

    <by>but got the following <br>0 bytes <by>instead:

       []


"#, actual);

}


#[test]
fn test_multiple_errors_text_body() {

    let actual = {
        API::post("/echo")
            .with_header(ContentType(
                Mime(TopLevel::Text, SubLevel::Plain, vec![])
            ))
            .with_body("Hello World Message")
            .expected_status(StatusCode::NotFound)
            .expected_header(Server("Foo".to_string()))
            .expected_body("Hello World")
            .collect()
    };

    assert_fail!(r#"
<br>Response Failure: <bn>POST <by>request to \"<bn>http://localhost:4000<bn>/echo\" <by>returned <br>3 <by>error(s)

<bb> 1) <by>Response <by>status code does not match value, expected:

        \"<bg>404 Not Found\"

    <by>but got:

        \"<br>200 OK\"

<bb> 2) <by>Response <by>header \"<bb>Server\" <by>was expected <bg>to be present<by>, but <br>is missing<by>.

<bb> 3) <by>Response <by>text body does not match, expected:

        \"<bg>Hello World\"

    <by>but got:

        \"<br>Hello World Message\"

    <by>difference:

        \"Hello World <gbg>Message\"


"#, actual);

}

