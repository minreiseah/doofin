use twsapi::core::{
    common::*,
    client::EClient,
    errors::IBKRApiLibError,
    streamer::Streamer,
    wrapper::Wrapper,
    execution::Execution,
    order::{Order, OrderState, SoftDollarTier},
    contract::{Contract, ContractDetails, ContractDescription, DeltaNeutralContract}
};
use std::{
    sync::{Arc, Mutex},
    string::String,
    time::{Duration, UNIX_EPOCH},
    collections::HashSet
};
use log::*;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

pub fn sq(a: i32) -> i32 {
    a * a
}

pub struct NeoWrapper<T: Streamer + 'static> {
    pub client: Option<Arc<Mutex<EClient<NeoWrapper<T>>>>>,
    pub next_order_id: i32,
    account: String,
}

impl<T: Streamer> NeoWrapper<T> {
    pub fn new() -> Self {
        NeoWrapper {
            client: None,
            next_order_id: -1,
            account: "".to_string(), }
    }

    pub fn start_requests(&mut self) -> Result<(), IBKRApiLibError> {
        self.order_operations_req()?;
        Ok(())
    }

    // Write order_operations_req

    // TODO Implement methods
}

// TODO Store the logged output somewhere
// TODO Resolve the types for security_definition_option_parameter

/// Implementing the Wrapper trait from twsapi for NeoWrapper.
/// This logs everything, but at the moment the logged output isn't stored anywhere

impl<T> Wrapper for NeoWrapper<T>
where
    T: Streamer + 'static,
{
    fn error(&mut self, req_id: i32, error_code: i32, error_string: &str) {
        // When TWS API shits a brick
        error!(
            "req_id: {} ,error_code: {} , error_string:{}",
            req_id, error_code, error_string
        );
    }

    fn win_error(&mut self, text: &str, last_error: i32) {
        // Some form of error, idk
        error!("text: {} , last_error:{}", text, last_error);
    }

    fn connect_ack(&mut self) {
        // Acknowledge connection
        info!("Connected.");
    }

    fn market_data_type(&mut self, req_id: i32, market_data_type: i32) {
        // Logs market data type.
        info!(
            "market data type -- req_id: {}, market_data_type: {}",
            req_id, market_data_type
        );
    }

    fn tick_price(&mut self, req_id: i32, tick_type: TickType, price: f64, attrib: TickAttrib) {
        // Logs tick price
        info!(
            "tick price -- req_id: {}, tick_type: {}, price: {}, attrib: {}",
            req_id, tick_type, price, attrib
        );
    }

    fn tick_size(&mut self, req_id: i32, tick_type: TickType, size: i32) {
        // Logs tick size
        info!(
            "tick size -- req_id: {}, tick_type: {}, size: {}",
            req_id, tick_type, size
        );
    }

    fn tick_snapshot_end(&mut self, req_id: i32) {
        // Logs tick snapshot end
        info!("tick_snapshot_end -- req_id: {}", req_id);
    }

    fn tick_generic(&mut self, req_id: i32, tick_type: TickType, value: f64) {
        // Logs tick generic
        info!("tick_generic -- req_id: {}, tick_type: {}, value {}", req_id, tick_type, value);
    }

    fn tick_string(&mut self, req_id: i32, tick_type: TickType, value: &str) {
        // Logs tick string
        info!("tick_string -- req_id: {}, tick_type: {}, value: {}", req_id, tick_type, value);
    }

    fn tick_efp(
        &mut self,
        req_id: i32,
        tick_type: TickType,
        basis_points: f64,
        formatted_basis_points: &str,
        implied_future: f64,
        hold_days: i32,
        future_last_trade_date: &str,
        dividend_impact: f64,
        dividends_to_last_trade_date: f64,
    ) {
        // Logs tick efp
        info!(
            "tick_efp -- req_id: {}, tick_type: {}, basis_points: {}, formatted_basis_points: {},
            implied_future: {}, hold_days: {}, future_last_trade_date: {}, dividend_impact: {},
            dividends_to_last_trade_date: {}",
            req_id, tick_type, basis_points, formatted_basis_points, implied_future,
            hold_days, future_last_trade_date, dividend_impact, dividends_to_last_trade_date
        );
    }

    fn order_status(
        &mut self,
        order_id: i32,
        status: &str,
        filled: f64,
        remaining: f64,
        avg_fill_price: f64,
        perm_id: i32,
        parent_id: i32,
        last_fill_price: f64,
        client_id: i32,
        why_held: &str,
        mkt_cap_price: f64,
    ) {
        info!(
            "order_status -- order_id: {}, status: {}, filled: {}, remaining: {}, avg_fill_price: {},
            perm_id: {}, parent_id: {}, last_fill_price: {}, client_id: {}, why_held: {}, mkt_cap_price: {}",
            order_id, status, filled, remaining, avg_fill_price, perm_id,
            parent_id, last_fill_price, client_id, why_held, mkt_cap_price
        );
    }

    fn open_order(
        &mut self,
        order_id: i32,
        contract: Contract,
        order: Order,
        order_state: OrderState,
    ) {
        info!(
            "open_order -- order_id: {}, contract: {}, order: {}, order_state: {}",
            order_id, contract, order, order_state
        );
    }

    fn open_order_end(&mut self) {
        // Called at the end of a given request for open orders
        info!("open_order_end. (no parameters passed)");
    }

    fn connection_closed(&mut self) {
        // Called when TWS closes the socket, or TWS is shut down
        info!("connection_closed. (no parameters passed)");
    }

    fn update_account_value(&mut self, key: &str, val: &str, currency: &str, account_name: &str) {
        // Logs update account value
        info!(
            "update_account_value -- key: {}, val: {}, currency: {}, account_name: {}",
            key, val, currency, account_name
        );
    }

    fn update_portfolio(
        &mut self,
        contract: Contract,
        position: f64,
        market_price: f64,
        market_value: f64,
        average_cost: f64,
        unrealized_pnl: f64,
        realized_pnl: f64,
        account_name: &str,
    ) {
        info!(
            "update_portfolio -- contract: {}, position: {}, market_price: {}, market_value: {},
            average_cost: {}, unrealized_pnl: {}, realized_pnl: {}, account_name: {}",
            contract, position, market_price, market_value, average_cost,
            unrealized_pnl, realized_pnl, account_name
        );
    }

    fn update_account_time(&mut self, time_stamp: &str) {
        // Logs update account time
        info!("update_account_time -- time_stamp: {}", time_stamp);
    }

    fn next_valid_id(&mut self, order_id: i32) {
        // Logs next valid ID
        self.next_order_id = order_id;
        info!(
            "next_valid_id -- order_id: {}",
            order_id
        );

        if self.start_requests().is_err() {
            panic!("start_requests failed");
        }
    }

    fn contract_details(&mut self, req_id: i32, contract_details: ContractDetails) {
        // Logs contract details
        info!("contract_details -- req_id: {}, contract_details: {}", req_id, contract_details);
    }

    fn bond_contract_details(&mut self, req_id: i32, contract_details: ContractDetails) {
        // Logs bond contract details
        info!(
            "bond_contract_details -- req_id: {}, contract_details: {}",
            req_id, contract_details
        );
    }

    fn contract_details_end(&mut self, req_id: i32) {
        // Logs contract details end
        info!("contract_details_end -- req_id: {}", req_id);
    }

    fn exec_details(&mut self, req_id: i32, contract: Contract, execution: Execution) {
        // Logs exec_details
        info!("exec_details -- req_id: {}, contract: {}, execution: {}", req_id, contract, execution);
    }

    fn exec_details_end(&mut self, req_id: i32) {
        // Logs exec details end
        info!("exec_details_end -- req_id: {}", req_id);
    }

    fn update_mkt_depth(
        &mut self,
        req_id: i32,
        position: i32,
        operation: i32,
        side: i32,
        price: f64,
        size: i32,
    ) {
        info!(
            "update_mkt_depth -- req_id: {}, position: {}, operation: {}, side: {}, price: {}, size: {}",
            req_id, position, operation, side, price, size
        );
    }

    fn update_mkt_depth_l2(
        &mut self,
        req_id: i32,
        position: i32,
        market_maker: &str,
        operation: i32,
        side: i32,
        price: f64,
        size: i32,
        is_smart_depth: bool,
    ) {
        info!(
            "update_mkt_depth_l2 -- req_id: {}, position: {}, market_maker: {}, operation: {}, side: {},
            price: {}, size: {}, is_smart_depth: {}",
            req_id, position, market_maker, operation, side, price, size, is_smart_depth
        );
    }

    fn update_news_bulletin(
        &mut self,
        msg_id: i32,
        msg_type: i32,
        news_message: &str,
        origin_exch: &str,
    ) {
        info!(
            "update_news_bulletin -- msg_id: {}, msg_type: {}, news_message: {}, origin_exch: {}",
            msg_id, msg_type, news_message, origin_exch
        );
    }

    fn managed_accounts(&mut self, accounts_list: &str) {
        // Logs managed accounts
        info!("managed_accounts -- accounts_list: {}", accounts_list);
    }

    fn receive_fa(&mut self, fa_data: FaDataType, cxml: &str) {
        // Logs received fa
        info!("receive_fa -- fa_data: {}, cxml: {}", fa_data, cxml);
    }

    fn historical_data(&mut self, req_id: i32, bar: BarData) {
        // Logs historical data
        info!("historical_data -- req_id: {}, bar: {}", req_id, bar);
    }

    fn historical_data_end(&mut self, req_id: i32, start: &str, end: &str) {
        // Logs historical data end
        info!("historical_data_end -- req_id: {}, start: {}, end: {}", req_id, start, end);
    }

    fn account_download_end(&mut self, account_name: &str) {
        // Logs account download end
        info!("account_download_end -- account_name: {}", account_name);
    }

    fn scanner_parameters(&mut self, xml: &str) {
        // Logs scanner parameters
        info!("scanner_parameters -- xml: {}", xml);
    }

    fn scanner_data(
        &mut self,
        req_id: i32,
        rank: i32,
        contract_details: ContractDetails,
        distance: &str,
        benchmark: &str,
        projection: &str,
        legs_str: &str,
    ) {
        // Logs scanner data
        info!(
            "scanner_data -- req_id: {}, rank: {}, contract_details: {}, distance: {},
            benchmark: {}, projection: {}, legs_str: {}",
            req_id, rank, contract_details, distance, benchmark, projection, legs_str
        );
    }

    fn scanner_data_end(&mut self, req_id: i32) {
        // Logs scanner data end
        info!("scanner_data_end -- req_id: {}", req_id);
    }

    fn realtime_bar(&mut self, req_id: i32, bar: RealTimeBar) {
        // Logs realtime bar
        info!(
            "realtime_bar -- req_id: {}, date_time: {}, open: {}, high: {}, low: {},
            close: {}, volume: {}, wap: {}, count: {}",
            req_id, bar.date_time, bar.open, bar.high, bar.low, bar.close,
            bar.volume, bar.wap, bar.count
        );
    }

    fn current_time(&mut self, time: i64) {
        // Creates a new SystemTime from the specified number of whole seconds
        let d = UNIX_EPOCH + Duration::from_secs(time as u64);
        // Create DateTime from SystemTime
        let datetime = DateTime::<Utc>::from(d);
        // Formats the combined date and time with the specified format string.
        let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
        info!("current_time -- time: {}", timestamp_str);
    }

    fn fundamental_data(&mut self, req_id: i32, data: &str) {
        // Logs fundamental data
        info!("fundamental_data -- req_id: {}, data: {}", req_id, data);
    }

    fn delta_neutral_validation(
        &mut self,
        req_id: i32,
        delta_neutral_contract: DeltaNeutralContract,
    ) {
        info!("delta_neutral_validation -- req_id: {}, delta_neutral_contract: {}", req_id, delta_neutral_contract);
    }

    fn commission_report(&mut self, commission_report: CommissionReport) {
        // Logs commission report
        info!("commission_report -- commission_report: {}", commission_report);
    }

    fn position(&mut self, account: &str, contract: Contract, position: f64, avg_cost: f64) {
        // Logs position
        info!(
            "position -- account: {}, contract: [{}], position: {}, avg_cost: {}",
            account, contract, position, avg_cost
        );
    }

    fn position_end(&mut self) {
        // Logs position end
        info!("position end. (no params)");
    }

    fn account_summary(
        &mut self,
        req_id: i32,
        account: &str,
        tag: &str,
        value: &str,
        currency: &str,
    ) {
        info!(
            "account_summary -- req_id: {}, account: {}, tag: {}, value: {}, currency: {}",
            req_id, account, tag, value, currency
        );
    }

    fn account_summary_end(&mut self, req_id: i32) {
        // Logs account summary end
        info!("account_summary_end -- req_id: {}", req_id);
    }

    fn verify_message_api(&mut self, api_data: &str) {
        // Logs verify message api
        info!("verify_message_api -- api_data: {}", api_data);
    }

    fn verify_completed(&mut self, is_successful: bool, error_text: &str) {
        // Logs verify completed
        info!("verify_completed -- is_successful: {}, error_text: {}", is_successful, error_text);
    }

    fn verify_and_auth_message_api(&mut self, api_data: &str, xyz_challange: &str) {
        // Logs verify and auth message api
        info!(
            "verify_and_auth_message_api -- api_data: {}, xyz_challenge: {}",
            api_data, xyz_challange
        );
    }

    fn verify_and_auth_completed(&mut self, is_successful: bool, error_text: &str) {
        // Logs verify and auth completed
        info!("verify_and_auth_completed -- is_successful: {}, error_text: {}", is_successful, error_text);
    }

    fn display_group_list(&mut self, req_id: i32, groups: &str) {
        // Logs display group list
        info!("display_group_list -- req_id: {}, groups: {}", req_id, groups);
    }

    fn display_group_updated(&mut self, req_id: i32, contract_info: &str) {
        // Logs display group updated
        info!("display_group_updated -- req_id: {}, contract_info: {}", req_id, contract_info);
    }

    fn position_multi(
        &mut self,
        req_id: i32,
        account: &str,
        model_code: &str,
        contract: Contract,
        pos: f64,
        avg_cost: f64,
    ) {
        info!(
            "position_multi -- req_id: {}, account: {}, model_code: {}, contract: {}, pos: {}, avg_cost: {}",
            req_id, account, model_code, contract, pos, avg_cost
        );
    }

    fn position_multi_end(&mut self, req_id: i32) {
        // Logs position multi end
        info!("position_multi_end -- req_id: {}", req_id);
    }

    fn account_update_multi(
        &mut self,
        req_id: i32,
        account: &str,
        model_code: &str,
        key: &str,
        value: &str,
        currency: &str,
    ) {
        info!(
            "account_update_multi -- req_id: {}, account: {}, model_code: {}, key: {}, value: {}, currency: {}",
            req_id, account, model_code, key, value, currency
        );
    }

    fn account_update_multi_end(&mut self, req_id: i32) {
        // Logs account update multi end
        info!("account_update_multi_end -- req_id: {}", req_id);
    }

    fn tick_option_computation(
        &mut self,
        req_id: i32,
        tick_type: TickType,
        implied_vol: f64,
        delta: f64,
        opt_price: f64,
        pv_dividend: f64,
        gamma: f64,
        vega: f64,
        theta: f64,
        und_price: f64,
    ) {
        info!(
            "tick_option_computation -- req_id: {}, tick_type: {}, implied_vol: {}, delta: {}, \
             opt_price: {}, pv_dividend: {},  gamma: {}, vega: {}, theta: {}, und_price: {}",
            req_id, tick_type, implied_vol, delta, opt_price, pv_dividend,
            gamma, vega, theta, und_price
        );
    }

    // TODO Figure out why this is throwing an error
    // It says bigdecimal::BigDecimal is required, but that is literally what is inside
    fn security_definition_option_parameter(
        &mut self,
        req_id: i32,
        exchange: &str,
        underlying_con_id: i32,
        trading_class: &str,
        multiplier: &str,
        expirations: HashSet<String>,
        strikes: HashSet<BigDecimal>,
    ) {
        info!(
            "tick_option_computation -- req_id: {}, exchange: {}, underlying_con_id: {}, \
             trading_class: {}, multiplier: {}, expirations: {:?},  strikes: {:?}",
            req_id,
            exchange,
            underlying_con_id,
            trading_class,
            multiplier,
            expirations
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<&str>>(),
            strikes
                .iter()
                .map(|x| x.clone())
                .collect::<Vec<BigDecimal>>()
        ); 
    }

    fn security_definition_option_parameter_end(&mut self, req_id: i32) {
        // Log security definition option parameter end
        info!("security_definition_option_parameter_end -- req_id: {}", req_id);
    }

    fn soft_dollar_tiers(&mut self, req_id: i32, tiers: Vec<SoftDollarTier>) {
        // Log soft dollar tiers
        info!("soft_dollar_tiers -- req_id: {}, tiers: {:?}", req_id, tiers);
    }

    fn family_codes(&mut self, family_codes: Vec<FamilyCode>) {
        // Log family codes
        info!("family_codes -- family_codes: {:?}", family_codes);
    }

    fn symbol_samples(&mut self, req_id: i32, contract_descriptions: Vec<ContractDescription>) {
        // Log symbol samples
        info!("symbol_samples -- req_id: {}, contract_descriptions: {:?}", req_id, contract_descriptions);
    }

    fn mkt_depth_exchanges(&mut self, depth_mkt_data_descriptions: Vec<DepthMktDataDescription>) {
        // Log mkt depth exchanges
        info!("mkt_depth_exchanges -- depth_mkt_data_descriptions: {:?}", depth_mkt_data_descriptions);
    }

    fn tick_news(
        &mut self,
        ticker_id: i32,
        time_stamp: i32,
        provider_code: &str,
        article_id: &str,
        headline: &str,
        extra_data: &str,
    ) {
        info!(
            "tick_news -- ticker_id: {}, time_stamp: {}, provider_code: {}, article_id: {}, \
             headline: {}, extra_data: {},",
            ticker_id, time_stamp, provider_code, article_id, headline, extra_data
        );
    }

    fn smart_components(&mut self, req_id: i32, smart_components: Vec<SmartComponent>) {
        // Log smart components
        info!("smart_components -- req_id: {}, smart_components: {:?}", req_id, smart_components);
    }

    fn tick_req_params(
        &mut self,
        ticker_id: i32,
        min_tick: f64,
        bbo_exchange: &str,
        snapshot_permissions: i32,
    ) {
        info!(
            "tick_req_params -- ticker_id: {}, min_tick: {}, bbo_exchange: {}, snapshot_permissions: {}",
            ticker_id, min_tick, bbo_exchange, snapshot_permissions
        );
    }

    fn news_providers(&mut self, news_providers: Vec<NewsProvider>) {
        // Log news providers
        info!("news_providers -- news_providers: {:?}", news_providers);
    }

    fn news_article(&mut self, request_id: i32, article_type: i32, article_text: &str) {
        // Log news article
        info!(
            "news_article -- request_id: {}, article_type: {}, article_text: {}",
            request_id, article_type, article_text
        );
    }

    fn historical_news(
        &mut self,
        request_id: i32,
        time: &str,
        provider_code: &str,
        article_id: &str,
        headline: &str,
    ) {
        info!(
            "historical_news -- request_id: {}, time: {}, provider_code: {}, article_id: {}, headline: {}",
            request_id, time, provider_code, article_id, headline
        );
    }

    fn historical_news_end(&mut self, request_id: i32, has_more: bool) {
        // Log historical news end
        info!("historical_news_end -- request_id: {}, has_more: {}", request_id, has_more);
    }

    fn head_timestamp(&mut self, req_id: i32, head_timestamp: &str) {
        // Log head timestamp
        info!("head_timestamp -- req_id: {}, head_timestamp: {}", req_id, head_timestamp);
    }

    fn histogram_data(&mut self, req_id: i32, items: Vec<twsapi::core::common::HistogramData>) {
        // Log histogram data
        info!("histogram_data -- req_id: {}, items: {:?}", req_id, items);
    }

    fn historical_data_update(&mut self, req_id: i32, bar: twsapi::core::common::BarData) {
        // Log historical data update
        info!("historical_data_update -- req_id: {}, bar: {}", req_id, bar);
    }

    fn reroute_mkt_data_req(&mut self, req_id: i32, con_id: i32, exchange: &str) {
        // Log reroute mkt data req
        info!("reroute_mkt_data_req -- req_id: {}, con_id: {}, exchange: {}", req_id, con_id, exchange);
    }

    fn reroute_mkt_depth_req(&mut self, req_id: i32, con_id: i32, exchange: &str) {
        // Log reroute mkt depth req
        info!("reroute_mkt_depth_req -- req_id: {}, con_id: {}, exchange: {}", req_id, con_id, exchange);
    }

    fn market_rule(&mut self, market_rule_id: i32, price_increments: Vec<twsapi::core::common::PriceIncrement>) {
        // Log market rule
        info!("market_rule -- market_rule_id: {}, price_increments: {:?}", market_rule_id, price_increments);
    }

    fn pnl(&mut self, req_id: i32, daily_pn_l: f64, unrealized_pn_l: f64, realized_pn_l: f64) {
        // Log pnl
        info!(
            "pnl -- req_id: {}, daily_pn_l: {}, unrealized_pn_l: {}, realized_pn_l: {})",
            req_id, daily_pn_l, unrealized_pn_l, realized_pn_l
        );
    }

    fn pnl_single(
        &mut self,
        req_id: i32,
        pos: i32,
        daily_pn_l: f64,
        unrealized_pn_l: f64,
        realized_pn_l: f64,
        value: f64,
    ) {
        info!(
            "pnl_single -- req_id: {}, pos: {}, daily_pn_l: {}, unrealized_pn_l: {}, realized_pn_l: {}, value: {})",
            req_id, pos, daily_pn_l, unrealized_pn_l, realized_pn_l, value
        );
    }

    fn historical_ticks(&mut self, req_id: i32, ticks: Vec<HistoricalTick>, done: bool) {
        // Log historical ticks
        info!("historical_ticks -- req_id: {}, ticks: {:?}, done: {}", req_id, ticks, done);
    }

    fn historical_ticks_bid_ask(
        &mut self,
        req_id: i32,
        ticks: Vec<twsapi::core::common::HistoricalTickBidAsk>,
        done: bool,
    ) {
        info!(
            "historical_ticks_bid_ask -- req_id: {}, ticks: {:?}, done: {}",
            req_id, ticks, done
        );
    }

    fn historical_ticks_last(&mut self, req_id: i32, ticks: Vec<HistoricalTickLast>, done: bool) {
        // Log historical ticks last
        info!("historical_ticks_last -- req_id: {}, ticks: {:?}, done: {}", req_id, ticks, done);
    }

    fn tick_by_tick_all_last(
        &mut self,
        req_id: i32,
        tick_type: TickByTickType,
        time: i64,
        price: f64,
        size: i32,
        tick_attrib_last: TickAttribLast,
        exchange: &str,
        special_conditions: &str,
    ) {
        info!(
            "tick_by_tick_all_last -- req_id: {}, tick_type: {:?}, time: {}, price: {}, size: {}, \
             tick_attrib_last: {}, exchange: {}, special_conditions: {}",
            req_id, tick_type, time, price, size, tick_attrib_last, exchange, special_conditions
        );
    }

    fn tick_by_tick_bid_ask(
        &mut self,
        req_id: i32,
        time: i64,
        bid_price: f64,
        ask_price: f64,
        bid_size: i32,
        ask_size: i32,
        tick_attrib_bid_ask: TickAttribBidAsk,
    ) {
        info!(
            "tick_by_tick_bid_ask -- req_id: {}, time: {}, bid_price: {}, ask_price: {}, bid_size: {}, \
             ask_size: {}, tick_attrib_last: {}",
            req_id, time, bid_price, ask_price, bid_size, ask_size, tick_attrib_bid_ask
        );
    }

    fn tick_by_tick_mid_point(&mut self, req_id: i32, time: i64, mid_point: f64) {
        // Log tick by tick mid point
        info!(
            "tick_by_tick_mid_point -- req_id: {}, time: {}, mid_point: {}",
            req_id, time, mid_point
        );
    }

    fn order_bound(&mut self, req_id: i32, api_client_id: i32, api_order_id: i32) {
        // Log order bound
        info!(
            "order_bound -- req_id: {}, api_client_id: {}, api_order_id: {}",
            req_id, api_client_id, api_order_id
        );
    }

    fn completed_order(&mut self, contract: Contract, order: Order, order_state: OrderState) {
        // Log completed order
        info!(
            "completed_order -- contract: [{}], order: [{}], order_state: [{}]",
            contract, order, order_state
        );
    }

    fn completed_orders_end(&mut self) {
        // Log completed orders end
        info!("completed_orders_end -- (no parameters for this message)");
    }
}