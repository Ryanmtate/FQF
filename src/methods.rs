/// Calculate the net present value of a series of cash flows, discounted by a rate
pub fn net_present_value(initial_cost: f64, cash_flows: Vec<f64>, discount_rate: f64) -> f64 {
    let mut npv = 0.0;
    let mut t = 1.0;

    for cash_flow in cash_flows {
        npv += cash_flow / (1.0 + discount_rate).powf(t);
        t += 1.0;
    }

    // Initial cash flow is a capital outlay and should be subtractive.
    npv - initial_cost.abs()
}

/// Calculate the internal rate of return of a series of cash flows
pub fn internal_rate_of_return(initial_cost: f64, cash_flows: Vec<f64>) -> f64 {
    let mut irr = 0.0;
    let mut npv = net_present_value(initial_cost, cash_flows.clone(), irr);

    while npv > 0.0001 {
        npv = net_present_value(initial_cost, cash_flows.clone(), irr);
        irr += 0.0001;
    }

    irr
}
