static EXPIRATION_DURATION: std::time::Duration = std::time::Duration::from_secs(1 * 24 * 60 * 60); // sessions expire after 1 day

pub struct Session {
    pub id: [u8; 4],
    pub created: std::time::Instant,
    pub hash: [u8; 8]
}

// check if a current session is valid against an old session
pub fn validate(session: Session, current: Session) -> bool {
    return session.created + EXPIRATION_DURATION > current && session.hash == current.hash;
}