pub const URI: &str = "text:///pi.txt";
const NAME: &str = "Value of pi";
const MIME_TYPE: &str = "text/plain";
const DATA: &str = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";
const SIZE: usize = DATA.len();

pub fn info()
-> Result<domain::entity::resource::ResourceInfo, domain::entity::resource::ResourceError> {
    Ok(domain::entity::resource::ResourceInfo {
        uri: URI.into(),
        name: NAME.into(),
        description: None,
        mime_type: Some(MIME_TYPE.into()),
        annotations: None,
        size: Some(SIZE),
    })
}

pub async fn read()
-> Result<domain::entity::resource::ReadResourceResult, domain::entity::resource::ResourceError> {
    Ok(domain::entity::resource::ReadResourceResult {
        contents: vec![domain::entity::resource::ResourceContent {
            uri: URI.into(),
            mime_type: Some(MIME_TYPE.into()),
            data: domain::entity::resource::ResourceData::Text(DATA.into()),
        }],
    })
}
