use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct DataModel{
    pub data:String,
}

#[allow(non_snake_case)]
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct GlobalConfigModel{
    pub db_host:String,
    pub db_port:u16,
    pub db_name:String,
    pub db_user_name:String,
    pub db_password:String,
    pub api_port:u16,
    pub toggle_log:i32,
    pub log_file_path:String,
    pub error_log:i32,
    pub io_log:i32,
}
#[allow(non_snake_case)]
#[derive(Serialize,Deserialize,Debug)]
pub struct DateModel{
    pub start_date:String,
    pub end_date:String,
}
#[allow(non_snake_case)]
#[derive(Serialize,Deserialize,Debug)]
pub struct TopNOverall{
    pub numbers:i32,
    pub start_date:String,
    pub end_date:String,
}
#[allow(non_snake_case)]
#[derive(Serialize,Deserialize,Debug)]
pub struct TopNFilter{
    pub numbers:i32,
    pub filterby:String,
    pub start_date:String,
    pub end_date:String,
}



