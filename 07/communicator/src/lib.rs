#[allow(dead_code)]
mod client0 {
    fn connect() {
    }
}

#[allow(dead_code)]
mod network0 {
    fn connect() {
    }
}

#[allow(dead_code)]
mod network1 {
    fn connect() {
    }
    mod client {
        fn connect() {
        }
    }
}

#[allow(dead_code)]
mod client2 {
    fn connect() {
    }
}

#[allow(dead_code)]
mod network2 {
    fn connect() {
    }
    mod server {
        fn connect() {
        }
    }
}

pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
