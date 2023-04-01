fn main() {
    enum Status {
        Value(u32),
        Stop
    }

    let v = Status::Value(3);

    let list_of_statuses: Vec<Status> =
        (0u32..20)
            .map(Status::Value)
            .collect();

}