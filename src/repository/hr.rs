use tokio;
use crate::database::db::DB;
use tokio_postgres::{NoTls};
use crate::models::{Hr};

pub async fn gets() ->Result<Vec<Hr>,tokio_postgres::Error>{
    let (client,connection) = tokio_postgres::connect(DB::url().url,NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error : {}",e);
        }
    });
    let mut emps:Vec<Hr> = vec![];
    let command = format!("select employee.emp_name,department.dep_name,hr.salary from hr 
                                            left join employee on hr.emp_id = employee.emp_id
                                            left join department on hr.dep_id = department.dep_id;");
    let rows = client.query(&command,&[]).await?;
    for row in rows {
        emps.push(Hr::new(row.get(0),row.get(1),row.get(2)));
    }
    Ok(emps)
}
pub async fn getbyempid(empid:String) ->Result<Hr,tokio_postgres::Error>{
    let (client,connection) = tokio_postgres::connect(DB::url().url,NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error : {}",e);
        }
    });
    let mut emp :Hr = Hr::default();
    let command = format!("select employee.emp_name,department.dep_name,hr.salary from hr 
                                            left join employee on hr.emp_id = employee.emp_id
                                            left join department on hr.dep_id = department.dep_id where hr.emp_id = $1");
    let rows = client.query(&command,&[&empid]).await?;
    for row in rows {
        emp = Hr::new(row.get(0),row.get(1),row.get(2));
    }
    Ok(emp)
}
pub async fn update(emp:Hr) ->Result<u64,tokio_postgres::Error>{
    let (client,connection) = tokio_postgres::connect(DB::url().url,NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error : {}",e);
        }
    });

    let command = format!("update hr set dep_id = $1,salary = $2 where emp_id = $3");
    let row = client.execute(&command, &[&emp.dep_id,&emp.salary,&emp.emp_id]).await?;
    Ok(row)
}
pub async fn insert(emp:Hr)->Result<u64,tokio_postgres::Error>{
    let (client,connection) = tokio_postgres::connect(DB::url().url,NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error : {}",e);
        }
    });
    let command = format!("insert into hr (emp_id,dep_id,salary) values ($1,$2,$3)");
    let row = client.execute(&command, &[&emp.emp_id,&emp.dep_id,&emp.salary]).await?;
    Ok(row)
}
pub async fn delete(empid:String)->Result<u64,tokio_postgres::Error>{
    let (client,connection) = tokio_postgres::connect(DB::url().url,NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error : {}",e);
        }
    });
    let command = format!("delete from hr where emp_id = $1");
    let row = client.execute(&command, &[&empid]).await?;
    Ok(row)
}