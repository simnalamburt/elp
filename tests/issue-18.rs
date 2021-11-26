use elp::ParsingErrors;

type TestResult = Result<(), ParsingErrors<'static>>;

#[test]
fn test_corner_cases() -> TestResult {
    elp::parse_record(r#"2011-11-11T11:11:11.000000Z REDACTED_ELB_NAME 111.11.11.111:11111 11.1.1.1:11 0.00004 0.012272 0.000039 404 404 0 120 "GET https://11.11.111.111:111/login.action HTTP/1.1" "\"Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv" ECDHE-RSA-AES128-GCM-SHA256 TLSv1.2"#)?;
    Ok(())
}
