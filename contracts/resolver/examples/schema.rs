use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, export_schema_with_title, remove_schemas, schema_for};

use gid::resolver::{
    InstantiateMsg,
    ExecuteMsg,
    QueryMsg,
    MigrateMsg,
    AddressResponse,
    TextDataResponse,
    ContentHashResponse,
    ConfigResponse
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(MigrateMsg), &out_dir);
    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema_with_title(&schema_for!(ExecuteMsg), &out_dir, "ExecuteMsg");
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(AddressResponse), &out_dir);
    export_schema(&schema_for!(TextDataResponse), &out_dir);
    export_schema(&schema_for!(ContentHashResponse), &out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
}
