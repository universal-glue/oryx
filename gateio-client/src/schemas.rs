pub enum TradeStatus {
    Untradable,
    Buyable,
    Sellable,
    Tradeable,
}

pub enum OrderStatus {
    Open,
    Closed,
    Cancelled,
}

pub enum OrderTimeInForce {
    Gtc,
    Ioc,
    Poc,
}

pub struct Currency {
    pub currency:String,
    pub delisted:bool,
    pub withdraw_disabled:bool,
    pub withdraw_delayed:bool,
    pub deposit_disabled:bool,
    pub trade_disabled:bool,
}

pub struct CurrencyPair {
    pub id: String,
    pub base: String,
    pub quote: String,
    pub fee: String,
    pub min_base_amount: String,
    pub min_quote_amount: String,
    pub amount_precision: u32,
    pub precision: u32,
    pub trade_status: TradeStatus,
}

pub struct Order {
    pub id: String,
    pub text: String,
    pub create_time: String,
    pub update_time: String,
    pub create_time_ms: u64,
    pub update_time_ms: u64,
    pub status: OrderStatus,
    pub currency_pair: String,
    pub r#type: String,
    pub account: String,
    pub side: String,
    pub amount: String,
    pub price: String,
    pub time_in_force: OrderTimeInForce,
    pub iceberg: String,
    pub auto_borrow: bool,
    pub auto_repay: bool,
    pub left: String,
    pub fill_price: String,
    pub filled_total: String,
    pub fee: String,
    pub fee_currency: String,
    pub point_fee: String,
    pub gt_fee: String,
    pub gt_discount: bool,
    pub rebated_fee: String,
    pub rebated_fee_currency: String,
}

pub enum LoanStatus {
    Open,
    Loaned,
    Finished,
    AutoRepaid,
}

pub enum LoanSide {
    Lend,
    Borrow,
}

pub struct Loan {
    pub id: String,
    pub create_time: String,
    pub expire_time: String,
    pub status: LoanStatus,
    pub side: LoanSide,
    pub currency: String,
    pub rate: String,
    pub amount: String,
    pub days: u32,
    pub auto_renew: bool,
    pub currency_pair: String,
    pub left: String,
    pub repaid: String,
    pub paid_interest: String,
    pub unpaid_interest: String,
    pub fee_rate: String,
    pub orig_id: String,
    pub text: String,
}

pub enum LoanRecordStatus {
    Loaned,
    Finished,
}

pub struct LoanRecord {
    pub id: String,
    pub loan_id: String,
    pub create_time: String,
    pub expire_time: String,
    pub status: LoanRecordStatus,
    pub borrow_user_id: String,
    pub currency: String,
    pub rate: String,
    pub amount: String,
    pub days: u32,
    pub auto_renew: bool,
    pub repaid: String,
    pub paid_interest: String,
    pub unpaid_interest: String,
}

pub struct Trigger {
    pub price: String,
    pub rule: String,
    pub expiration: u32,
}

pub enum SpotPriceTriggeredOrderStatus {
    ReadOnly,
    Open,
    Cancelled,
    Finish,
    Failed,
    Expired,
}

pub struct SpotPriceTriggeredOrder {
    pub trigger: Trigger,
    pub id: u64,
    pub user: u32,
    pub market: String,
    pub ctime: f64,
    pub ftime: f64,
    pub fired_order_id: u64,
    pub status: SpotPriceTriggeredOrderStatus,
    pub reason: String,
}
