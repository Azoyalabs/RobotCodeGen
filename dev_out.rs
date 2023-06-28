pub trait RobotCodeGenRobot {
	pub fn limit_order(app: &mut App, market_id : u64 , price : Decimal , funds: Vec<Coin>);
	pub fn market_order(app: &mut App, market_id : u64 , funds: Vec<Coin>);
	pub fn remove_limit_order(app: &mut App, market_id : u64 , price : Decimal , funds: Vec<Coin>);
	pub fn get_admin(app: &App, ) -> GetAdminResponse;
	pub fn get_markets(app: &App, ) -> GetMarketsResponse;
	pub fn get_user_bids(app: &App, user_address : Addr , target_market : Option < u64 > ,) -> GetUserBidsResponse;
	pub fn get_user_asks(app: &App, user_address : Addr , target_market : Option < u64 > ,) -> GetUserAsksResponse;
	pub fn get_user_orders(app: &App, user_address : Addr , target_market : Option < u64 > ,) -> GetUserOrdersResponse;
	pub fn get_market_book(app: &App, market_id : u64 , nb_levels : u32 ,) -> GetMarketBookResponse;
}

impl RobotCodeGenRobot for Robot {
	fn limit_order(app: &mut App, contract: &Addr, caller: &Addr, market_id : u64 , price : Decimal , funds: Vec<Coin>){
		let msg = LimitOrder {market_id , price ,};
		app.execute_contract(caller.to_owned(), contract.to_owned(), &msg, &funds).unwrap();
	}
	fn market_order(app: &mut App, contract: &Addr, caller: &Addr, market_id : u64 , funds: Vec<Coin>){
		let msg = MarketOrder {market_id ,};
		app.execute_contract(caller.to_owned(), contract.to_owned(), &msg, &funds).unwrap();
	}
	fn remove_limit_order(app: &mut App, contract: &Addr, caller: &Addr, market_id : u64 , price : Decimal , funds: Vec<Coin>){
		let msg = RemoveLimitOrder {market_id , price ,};
		app.execute_contract(caller.to_owned(), contract.to_owned(), &msg, &funds).unwrap();
	}
	fn get_admin(app: &App, contract: &Addr, ) -> GetAdminResponse{
		let msg = GetAdmin {};
		return app.wrap().query_wasm_smart(contract.to_owned(), &msg).unwrap();
	}
	fn get_markets(app: &App, contract: &Addr, ) -> GetMarketsResponse{
		let msg = GetMarkets {};
		return app.wrap().query_wasm_smart(contract.to_owned(), &msg).unwrap();
	}
	fn get_user_bids(app: &App, contract: &Addr, user_address : Addr , target_market : Option < u64 > ,) -> GetUserBidsResponse{
		let msg = GetUserBids {user_address , target_market ,};
		return app.wrap().query_wasm_smart(contract.to_owned(), &msg).unwrap();
	}
	fn get_user_asks(app: &App, contract: &Addr, user_address : Addr , target_market : Option < u64 > ,) -> GetUserAsksResponse{
		let msg = GetUserAsks {user_address , target_market ,};
		return app.wrap().query_wasm_smart(contract.to_owned(), &msg).unwrap();
	}
	fn get_user_orders(app: &App, contract: &Addr, user_address : Addr , target_market : Option < u64 > ,) -> GetUserOrdersResponse{
		let msg = GetUserOrders {user_address , target_market ,};
		return app.wrap().query_wasm_smart(contract.to_owned(), &msg).unwrap();
	}
	fn get_market_book(app: &App, contract: &Addr, market_id : u64 , nb_levels : u32 ,) -> GetMarketBookResponse{
		let msg = GetMarketBook {market_id , nb_levels ,};
		return app.wrap().query_wasm_smart(contract.to_owned(), &msg).unwrap();
	}
}