use harsh::Harsh;
use responses::ErrorResponses;

pub fn extract_id(harsh: &Harsh, hid: &str) -> Result<u32, ErrorResponses> {
    let ids:Vec<u64> = match harsh.decode(&hid) {
        Some(vs) => vs,
        None => return Err(ErrorResponses::NotFound)
    };
    if ids.len() != 1 {
        return Err(ErrorResponses::NotFound);
    }
    Ok(ids[0] as u32)
}