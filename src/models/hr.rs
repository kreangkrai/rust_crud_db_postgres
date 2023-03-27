use serde::{Deserialize,Serialize};
#[derive(Deserialize,Serialize,Debug,Clone)]

pub struct Hr {
    pub emp_id:String,
    pub dep_id:String,
    pub salary:f32,
}
impl Hr {
    pub fn new(emp_id:String,dep_id:String,salary:f32)->Self{
        Hr {
            emp_id : emp_id,
            dep_id : dep_id,
            salary :salary
        }
    }
}
impl Default for Hr {
    fn default() -> Self {
        Self { emp_id: Default::default(), dep_id: Default::default(), salary: Default::default() }
    }
}