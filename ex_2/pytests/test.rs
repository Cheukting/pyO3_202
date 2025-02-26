mod tests {
    use pyo3::prelude::*;
    use py202::make_order;

    #[pyo3_async_runtimes::tokio::test]
    async fn test_async_sleep() -> PyResult<()> {
        make_order("burger soda burger".to_string()).await;
        Ok(())
    }
}

#[pyo3_async_runtimes::tokio::main]
async fn main() -> pyo3::PyResult<()> {
    pyo3_async_runtimes::testing::main().await
}