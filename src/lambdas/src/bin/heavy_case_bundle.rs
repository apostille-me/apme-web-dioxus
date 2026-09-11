use lambda_runtime::Error;
use scintilla_route_lambdas::{run, RouteSpec};

const ROUTE: RouteSpec = RouteSpec::new(
    "heavy_case_bundle",
    "POST",
    "/api/heavy/case-bundle",
    "case_id",
);

#[tokio::main]
async fn main() -> Result<(), Error> {
    return run(ROUTE).await;
}
