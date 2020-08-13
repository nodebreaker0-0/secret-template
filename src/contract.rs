use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier,
    StdResult, Storage,
};

use crate::msg::{SumResponse, InitMsg, HandleMsg, QueryMsg};
use crate::state::{config, config_read, State};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State{
        v1: 0,
        v2: 0,
        sum: 0,
    };
    config(&mut deps.storage).save(&state)?;
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Add { v1, v2 } => try_add(deps, v1,v2),
    }
}

pub fn try_add<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    v1: i32,
    v2: i32,
) -> StdResult<HandleResponse> {
    config(&mut deps.storage).update(|mut state| {
        state.v1 = v1;
        state.v2 = v2;
        state.sum = state.v1 + state.v2;
        Ok(state)
    })?;

    Ok(HandleResponse::default())
}


pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetSum {} => to_binary(&query_sum(deps)?),
    }
}

fn query_sum<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<SumResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(SumResponse { sum: state.sum })
}