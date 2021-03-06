header! {
    #[doc="`User-Agent` header, defined in"]
    #[doc="[RFC7231](http://tools.ietf.org/html/rfc7231#section-5.5.3)"]
    #[doc=""]
    #[doc="The `User-Agent` header field contains information about the user"]
    #[doc="agent originating the request, which is often used by servers to help"]
    #[doc="identify the scope of reported interoperability problems, to work"]
    #[doc="around or tailor responses to avoid particular user agent"]
    #[doc="limitations, and for analytics regarding browser or operating system"]
    #[doc="use.  A user agent SHOULD send a User-Agent field in each request"]
    #[doc="unless specifically configured not to do so."]
    #[doc=""]
    #[doc="# ABNF"]
    #[doc="```plain"]
    #[doc="User-Agent = product *( RWS ( product / comment ) )"]
    #[doc="product         = token [\"/\" product-version]"]
    #[doc="product-version = token"]
    #[doc="```"]
    // TODO: Maybe write parsing according to the spec? (Split the String)
    (UserAgent, "User-Agent") => [String]
}

#[test] fn test_format() {
    use std::borrow::ToOwned;
    use header::Headers;
    let mut head = Headers::new();
    head.set(UserAgent("Bunnies".to_owned()));
    assert!(head.to_string() == "User-Agent: Bunnies\r\n".to_owned());
}
