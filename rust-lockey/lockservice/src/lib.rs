use std::error::Error;
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use crate::descriptors::Descriptor;
use crate::errors::LockServiceErrors;

pub mod descriptors;
pub mod errors;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn callout() {
    println!("Im the lockservice, here at your service")
}

/*

LockService is an implementation of the backend of a Lock server.
This is responsible for providing functions that enable a client
to acquire a lock based on one or a set of parameters and also
release them when needed.

V1 of this will just be this simple procedure as mentioned above.

V2 might have contention.

V3 might have timers associated with the locks and auto release.

*/

pub struct LockService {

    /*

    This contains all the details of the locks for atleast V1.

    */

    safe_lock_map: Arc<Mutex<HashMap<String, String>>>
}

/*

The implementation of LockService has 3 functions.

new() to create an new instance of the LockService for the server to work on.

acquire(<parameters>) for acquire a new lock based on the parameters.

release(<parameters>) for releasing an acquired lock based on the parameters.

*/

impl LockService {

    /*

    new returns a new instance of the LockService.

    */

    pub fn new() -> LockService {
        let safe_map : Arc<Mutex<HashMap<String,String>>> = Arc::new(Mutex::new(HashMap::new()));
        LockService{
            safe_lock_map: safe_map
        }
    }

    /*

    acquire takes in the descriptors and if that object isn't being used,
    allocates a lock for this user.

    V1 allows a user to acquire only one lock at a time.

    V2 allows more than one lock per user ID.

    */

    pub fn acquire(&mut self, id: Descriptor) -> Result<(), Box<dyn Error>> {

        let mut map = self.safe_lock_map.lock().unwrap();
        if map.contains_key(&id.user_id) {
            return Err(Box::new(LockServiceErrors::ObjectAlreadyLocked));
        }

        map.insert(id.user_id, id.obj_id);

        println!("{:?}", map);
        Ok(())
    }

    /*

    release takes in the descriptors and if that object is in the map, it releases
    that lock.

    V1 doesn't care about if the user is the one who took the lock.

    V2 will care about if the user was the one who acquired the lock along with the multiple users thing.

    */

    pub fn release(&mut self, id: Descriptor) -> Result<(), Box<dyn Error>> {

        let mut map = self.safe_lock_map.lock().unwrap();

        if !map.contains_key(&id.user_id) {
            return Err(Box::new(LockServiceErrors::ObjectNotLocked));
        }

        map.remove(&id.user_id);

        println!("{:?}", map);
        Ok(())
    }
}

#[cfg(test)]

/*

These are the tests for the lockservice cargo package.

*/

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }


}
