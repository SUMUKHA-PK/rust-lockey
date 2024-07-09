use std::error::Error;
use lockservice::descriptors::Descriptor;
use lockservice::errors::LockServiceErrors;
use lockservice::LockService;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn callout() {
    println!("Im the lockclient, here at your client")
}

/*

LockClient is responsible for serving all the incoming requests in a
multithreaded manner. Not clear of this implementation rn,

so V1 will just call the acquire function of LockService and respond
with what it gets back.

V2 can probably go multithreaded.

V3 can be probably over the network.

*/
pub struct LockClient {
    lock_service: LockService
}

impl LockClient {

    pub fn new() -> LockClient {
        LockClient {
            lock_service: LockService::new()
        }
    }

    pub fn acquire(&mut self, user_id: String, object_id: String) -> Result<(), Box<dyn Error>> {

        let desc = new_descriptor(user_id, object_id);
        self.lock_service.acquire(desc)
    }

    pub fn release(&mut self, user_id: String, object_id: String) -> Result<(), Box<dyn Error>> {

        let desc = new_descriptor(user_id, object_id);
        self.lock_service.release(desc)
    }
}

pub fn new_descriptor(user_id: String, obj_id: String) -> Descriptor {
    Descriptor {
        user_id,
        obj_id
    }
}

#[cfg(test)]
/*

These are the tests for the lockclient cargo crate.

*/

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn acquire_test() {

        let mut lc = LockClient::new();

        // First acquisition should succeed
        assert!(lc.acquire("user1".to_string(), "object1".to_string()).is_ok());

        // Second acquisition should fail with ObjectAlreadyExists error
        match lc.acquire("user1".to_string(), "object1".to_string()) {
            Ok(_) => panic!("Expected an error, but got Ok"),
            Err(e) => assert_eq!(e.downcast().unwrap(), Box::new(LockServiceErrors::ObjectAlreadyLocked)),
        }

    }

    #[test]
    fn release_test() {
        let mut lc = LockClient::new();

        // First release should fail with ObjectNotLocked error
        match lc.release("user1".to_string(), "object1".to_string()) {
            Ok(_) => panic!("Expected an error, but got Ok"),
            Err(e) => assert_eq!(e.downcast().unwrap(), Box::new(LockServiceErrors::ObjectNotLocked)),
        }

        assert!(lc.acquire("user1".to_string(), "object1".to_string()).is_ok());

        assert!(lc.release("user1".to_string(), "onject1".to_string()).is_ok());
    }
}
