use uuid::Uuid;

pub fn must_new() -> String {
    Uuid::now_v7().to_string()
}
