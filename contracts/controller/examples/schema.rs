use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, export_schema_with_title, remove_schemas, schema_for};

use controller::msg::{
    InstantiateMsg,
    ExecuteMsg,
    QueryMsg,
    MigrateMsg,
    GetCommitmentResponse,
    CommitmentTimestampResponse,
    RentPriceResponse,
    MaxCommitmentAgeResponse,
    MinCommitmentAgeResponse,
    MinRegistrationDurationResponse,
    IsValidNameResponse,
    TokenIdResponse,
    NodehashResponse,
    NodeInfoResponse,
    OwnerResponse,
    RegistrarResponse,
    PriceResponse
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
    export_schema(&schema_for!(GetCommitmentResponse), &out_dir);
    export_schema(&schema_for!(CommitmentTimestampResponse), &out_dir);
    export_schema(&schema_for!(RentPriceResponse), &out_dir);
    export_schema(&schema_for!(MaxCommitmentAgeResponse), &out_dir);
    export_schema(&schema_for!(MinCommitmentAgeResponse), &out_dir);
    export_schema(&schema_for!(MinRegistrationDurationResponse), &out_dir);
    export_schema(&schema_for!(IsValidNameResponse), &out_dir);
    export_schema(&schema_for!(TokenIdResponse), &out_dir);
    export_schema(&schema_for!(NodehashResponse), &out_dir);
    export_schema(&schema_for!(NodeInfoResponse), &out_dir);
    export_schema(&schema_for!(OwnerResponse), &out_dir);
    export_schema(&schema_for!(RegistrarResponse), &out_dir);
    export_schema(&schema_for!(PriceResponse), &out_dir);
}
