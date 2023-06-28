## RobotCodeGen  


### Overview  

Testing in CosmWasm entails a lot of boilerplate when using the cw_multi_test, including: 
- creating messages for execution and query 
- cloning addresses when making calls (copy is not implemented on Address type) 

We can get rid of part of this boilerplate by parsing the messages definition and generating methods on a custom struct through a build script.  


### Usage  
The code generation process is based on the organistion of our [Contract Template](https://github.com/Azoyalabs/ContractTemplate_CosmWasm). 
To enable code generation, you must create a build.rs file and use the generate_robot_code function from this crate.  

Then you can import the Robot struct definition, and the trait that was just generated in your test file and start using the created methods.

You can check the the contract template for a usage example: 
- [build.rs file](https://github.com/Azoyalabs/ContractTemplate_CosmWasm/blob/main/build.rs)
- [Generated code](https://github.com/Azoyalabs/ContractTemplate_CosmWasm/blob/main/tests/common/cosmwasm_contract_template_robot.rs)


### Limitations  
Generics and unnamed enum fields are unsupported (for example Admin(AdminExecuteMsg)) but are planned features.   
We also plan to enable excluding certain variants from the generation process. 

### Sample  
Defined messages in our template are the following:  
```rust
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum AdminExecuteMsg {
    UpdateAdmin { new_admin: String },
}

#[cw_serde]
pub enum ExecuteMsg {
    SampleExecute {},

    Admin(AdminExecuteMsg),
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetAdminResponse)]
    GetAdmin {},

    // GetCount returns the current count as a json-encoded number
    #[returns(SampleQueryResponse)]
    SampleQuery {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct SampleQueryResponse {
    pub value: bool,
}

#[cw_serde]
pub struct GetAdminResponse {
    pub admin: Option<Addr>,
}
```


The following trait definition and implementation will be generated:  
```rust
use robot_code_gen::Robot;
use cosmwasm_std::{Addr, Coin};
use cw_multi_test::{App, Executor};
use cosmwasm_contract_template::msg::*;

pub trait CosmwasmContractTemplateRobot {
	fn sample_execute(&mut self, app: &mut App, contract: &Addr, caller: &Addr,  funds: Vec<Coin>);
	fn get_admin(&self, app: &App, contract: &Addr, ) -> GetAdminResponse;
	fn sample_query(&self, app: &App, contract: &Addr, ) -> SampleQueryResponse;
}

impl CosmwasmContractTemplateRobot for Robot {
	fn sample_execute(&mut self, app: &mut App, contract: &Addr, caller: &Addr,  funds: Vec<Coin>){
		let msg = ExecuteMsg::SampleExecute {};
		app.execute_contract(caller.to_owned(), contract.to_owned(), &msg, &funds).unwrap();
	}
	fn get_admin(&self, app: &App, contract: &Addr, ) -> GetAdminResponse{
		let msg = QueryMsg::GetAdmin {};
		return app.wrap().query_wasm_smart(contract.to_owned(), &msg).unwrap();
	}
	fn sample_query(&self, app: &App, contract: &Addr, ) -> SampleQueryResponse{
		let msg = QueryMsg::SampleQuery {};
		return app.wrap().query_wasm_smart(contract.to_owned(), &msg).unwrap();
	}
}
```