use crate::repository::{hr};
use crate::errors::MyError;
use crate::models::Hr;
pub async fn gets() -> Result<Vec<Hr>,MyError>{
    let emps = hr::gets().await?;
    Ok(emps)
}
pub async fn getbyempid(empid:String) -> Result<Hr,MyError>{
    let emp = hr::getbyempid(empid).await?;
    Ok(emp)
}
pub async fn update(emp:Hr) -> Result<u64,MyError>{
    let message = hr::update(emp).await?;
    Ok(message)
}
pub async fn insert(emp:Hr) -> Result<u64,MyError>{
    let message = hr::insert(emp).await?;
    Ok(message)
}
pub async fn delete(empid:String) -> Result<u64,MyError>{
    let message = hr::delete(empid).await?;
    Ok(message)
}