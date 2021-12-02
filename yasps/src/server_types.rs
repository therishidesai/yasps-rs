use libc;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ReqType {
    Publisher,
    Subscriber,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicReq<'a> {
    pub req_type: ReqType,
    pub topic: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicRes<'a> {
    pub topic: Option<&'a str>,
    //eventfd: libc::c_int,
    // TODO: implement the shared memory queue stuff
    // shm_queue_ptr: libc::c_int
}
