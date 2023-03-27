use test_db::handlers::hr;
//use test_db::models::Hr;
use tokio;

fn main() {
    //Get All
    // let emps = tokio::runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .block_on(hr::gets());
    // match emps {
    //     Ok(emp) => println!("{:#?}", emp),
    //     Err(e) => println!("Error : {:?}", e),
    // }

    //Get By Employee
    // let emp = tokio::runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .block_on(hr::getbyempid("EM03".to_string()));
    // match emp {
    //     Ok(em) => println!("{:#?}", em),
    //     Err(e) => println!("Error : {:?}", e),
    // }
    
    //Update
    // let data_emp = Hr::new("EM01".to_string(), "DP01".to_string(), 99999.99);
    // let message = tokio::runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .block_on(hr::update(data_emp));
    // match message {
    //     Ok(em) => println!("{:#?}", em),
    //     Err(e) => println!("Error : {:?}", e),
    // }

    //Insert
    // let data_emp = Hr::new("EM03".to_string(), "DP03".to_string(), 88888.99);
    // let message = tokio::runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .block_on(hr::insert(data_emp));
    // match message {
    //     Ok(em) => println!("{:#?}", em),
    //     Err(e) => println!("Error : {:?}", e),
    // }

    //delete
    // let message = tokio::runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .block_on(hr::delete("EM01".to_string()));
    // match message {
    //     Ok(em) => println!("{:#?}", em),
    //     Err(e) => println!("Error : {:?}", e),
    // }
}