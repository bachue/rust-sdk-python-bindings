mod credential;
mod etag;
pub mod exceptions;
mod http;
mod http_client;
mod upload_token;
mod utils;

pub use exceptions::QiniuApiCallError;

use exceptions::QiniuUserAgentInitializeError;
use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "qiniu_sdk_bindings")]
fn qiniu_sdk_bindings(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    pyo3_log::try_init().ok();
    exceptions::register(py, m)?;
    initialize_user_agent(py)?;

    m.add_submodule(etag::create_module(py)?)?;
    m.add_submodule(credential::create_module(py)?)?;
    m.add_submodule(upload_token::create_module(py)?)?;
    m.add_submodule(http::create_module(py)?)?;
    m.add_submodule(http_client::create_module(py)?)?;

    return Ok(());

    fn initialize_user_agent(py: Python<'_>) -> PyResult<()> {
        let version = py.version_info();
        qiniu_sdk::http::set_library_user_agent(
            format!(
                "/qiniu-sdk-python-bindings-{}/python-{}.{}.{}",
                env!("CARGO_PKG_VERSION"),
                version.major,
                version.minor,
                version.patch,
            )
            .into(),
        )
        .map_err(|_| {
            QiniuUserAgentInitializeError::new_err("Failed to initialize user agent for Qiniu SDK")
        })
    }
}
