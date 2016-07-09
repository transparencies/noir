#[macro_use] extern crate json;
#[macro_use] extern crate noir;
#[macro_use]
mod base_test;
test!();


// Form Uploads ---------------------------------------------------------------
#[test]
fn test_with_form_body_url_encoded() {

    let actual = {
        API::post("/form")
            .with_body(form! {
                "field" => "someValue",
                "array[]" => vec![1, 2, 3, 4, 5]
            })
            .expected_body("field=someValue&array%5B%5D=1&array%5B%5D=2&array%5B%5D=3&array%5B%5D=4&array%5B%5D=5")
            .collect()
    };

    assert_pass!(actual);

}

#[test]
fn test_with_form_body_url_encoded_trailing_comma() {

    let actual = {
        API::post("/form")
            .with_body(form! {
                "field" => "someValue",
                "array[]" => vec![1, 2, 3, 4, 5],
            })
            .expected_body("field=someValue&array%5B%5D=1&array%5B%5D=2&array%5B%5D=3&array%5B%5D=4&array%5B%5D=5")
            .collect()
    };

    assert_pass!(actual);

}

#[test]
fn test_with_form_body_url_multipart_vec_file() {

    let actual = {
        API::post("/form")
            .with_body(form! {
                "vec_file" => (
                    "file.bin",
                    Mime(TopLevel::Application, SubLevel::OctetStream, vec![]),
                    vec![1, 2, 3, 4, 5, 6, 7, 8]
                )
            })
            .expected_body("\r\n--<boundary>\r\nContent-Disposition: form-data; name=\"vec_file\"; filename=\"file.bin\"\r\nContent-Type: application/octet-stream\r\n\r\n\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\r\n--<boundary>--\r\n")
            .collect()
    };

    assert_pass!(actual);

}

#[test]
fn test_with_form_body_url_multipart_str_file() {

    let actual = {
        API::post("/form")
            .with_body(form! {
                "str_file" => (
                    "readme.md",
                    Mime(TopLevel::Text, SubLevel::Plain, vec![]),
                    "Hello World"
                )
            })
            .expected_body("\r\n--<boundary>\r\nContent-Disposition: form-data; name=\"str_file\"; filename=\"readme.md\"\r\nContent-Type: text/plain\r\n\r\nHello World\r\n--<boundary>--\r\n")
            .collect()
    };

    assert_pass!(actual);

}

#[test]
fn test_with_form_body_url_multipart_string_file() {

    let actual = {
        API::post("/form")
            .with_body(form! {
                "string_file" => (
                    "readme.md",
                    Mime(TopLevel::Text, SubLevel::Plain, vec![]),
                    "Hello World".to_string()
                )
            })
            .expected_body("\r\n--<boundary>\r\nContent-Disposition: form-data; name=\"string_file\"; filename=\"readme.md\"\r\nContent-Type: text/plain\r\n\r\nHello World\r\n--<boundary>--\r\n")
            .collect()
    };

    assert_pass!(actual);

}

#[test]
fn test_with_form_body_url_multipart_json_file() {

    let actual = {
        API::post("/form")
            .with_body(form! {
                "json_file" => (
                    "data.json",
                    Mime(TopLevel::Application, SubLevel::Json, vec![]),
                    object! {
                        "key" => "value"
                    }
                )
            })
            .expected_body("\r\n--<boundary>\r\nContent-Disposition: form-data; name=\"json_file\"; filename=\"data.json\"\r\nContent-Type: application/json\r\n\r\n{\"key\":\"value\"}\r\n--<boundary>--\r\n")
            .collect()
    };

    assert_pass!(actual);

}

#[test]
fn test_with_form_body_url_multipart_fs_file() {

    use std::fs::File;

    let actual = {
        API::post("/form")
            .with_body(form! {
                "fs_file" => (
                    "form_test.md",
                    Mime(TopLevel::Text, SubLevel::Plain, vec![]),
                    File::open("./tests/form_test.md").unwrap()
                )
            })
            .expected_body("\r\n--<boundary>\r\nContent-Disposition: form-data; name=\"fs_file\"; filename=\"form_test.md\"\r\nContent-Type: text/plain\r\n\r\nForm Test Data File\n\r\n--<boundary>--\r\n")
            .collect()
    };

    assert_pass!(actual);

}

