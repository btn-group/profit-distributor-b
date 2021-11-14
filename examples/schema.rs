use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use cw_profit_distributor_b::msg::{
    ProfitDistributorBHandleMsg, ProfitDistributorBInitMsg, ProfitDistributorBQueryAnswer,
    ProfitDistributorBQueryMsg, ProfitDistributorBReceiveMsg,
};
use std::env::current_dir;
use std::fs::create_dir_all;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(ProfitDistributorBHandleMsg), &out_dir);
    export_schema(&schema_for!(ProfitDistributorBInitMsg), &out_dir);
    export_schema(&schema_for!(ProfitDistributorBQueryAnswer), &out_dir);
    export_schema(&schema_for!(ProfitDistributorBQueryMsg), &out_dir);
    export_schema(&schema_for!(ProfitDistributorBReceiveMsg), &out_dir);
}
