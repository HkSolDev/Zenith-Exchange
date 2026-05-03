use db::DbPool;
use domain::{Balance, Order, User};
use rust_decimal::Decimal;
use sqlx;

pub async  fn check_order(pool:DbPool, order:Order)  {
// let begin = match DbPool::begin(&pool).await {
//     Ok(db) => db,
//     Err(_) => {println!("err while connection");
// Error("err while connectioDbPool::begin(&self)")}

   let amount_to_blocked:Decimal = order.price.checked_mul(order.qty).expect("Error in multiplying the price and the qty of the order") ;

let user_update = sqlx::query!(
 "UPDATE balances 
 SET free  = free - $1 ,
 locked = locked + $1
WHERE user_id = $2 AND 
symbol = $3 AND free >= $1",
amount_to_blocked, order.user_id, order.symbol
).execute(&pool).await;

// match user_update {
//     Ok(user_balance) => {
//         println!("User found: {:#?}", user_balance);
//         let amount_to_blocked:Decimal = order.price.checked_mul(order.qty).expect("Error in multiplying the price and the qty of the order") ;
//       if amount_to_blocked >= user_balance.free {
//         println!("Insufficient Free Balance");
//         return false
//       }
//       return true;
//     }
//     Err(_) => {
//         println!("User not found");
//         return false;
//     }
// };
    
}

  