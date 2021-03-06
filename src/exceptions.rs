use maybe_owned::MaybeOwned;
use pyo3::{
    create_exception,
    exceptions::{PyIOError, PyRuntimeError, PyTypeError, PyValueError},
    prelude::*,
};
use std::{io::Error as IoError, time::SystemTimeError};

pub(super) fn register(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add(
        "QiniuUserAgentInitializeError",
        py.get_type::<QiniuUserAgentInitializeError>(),
    )?;
    m.add(
        "QiniuInvalidPortError",
        py.get_type::<QiniuInvalidPortError>(),
    )?;
    m.add(
        "QiniuEmptyChainCredentialsProvider",
        py.get_type::<QiniuEmptyChainCredentialsProvider>(),
    )?;
    m.add(
        "QiniuEmptyRegionsProvider",
        py.get_type::<QiniuEmptyRegionsProvider>(),
    )?;
    m.add(
        "QiniuEmptyChainedResolver",
        py.get_type::<QiniuEmptyChainedResolver>(),
    )?;
    m.add("QiniuEmptyEndpoints", py.get_type::<QiniuEmptyEndpoints>())?;
    m.add(
        "QiniuUnsupportedTypeError",
        py.get_type::<QiniuUnsupportedTypeError>(),
    )?;
    m.add(
        "QiniuBodySizeMissingError",
        py.get_type::<QiniuBodySizeMissingError>(),
    )?;
    m.add(
        "QiniuInvalidConcurrency",
        py.get_type::<QiniuInvalidConcurrency>(),
    )?;
    m.add(
        "QiniuInvalidObjectSize",
        py.get_type::<QiniuInvalidObjectSize>(),
    )?;
    m.add(
        "QiniuInvalidPartSize",
        py.get_type::<QiniuInvalidPartSize>(),
    )?;
    m.add(
        "QiniuInvalidMultiply",
        py.get_type::<QiniuInvalidMultiply>(),
    )?;
    m.add(
        "QiniuInvalidLimitation",
        py.get_type::<QiniuInvalidLimitation>(),
    )?;
    m.add(
        "QiniuInvalidSourceKeyLengthError",
        py.get_type::<QiniuInvalidSourceKeyLengthError>(),
    )?;

    QiniuInvalidURLError::register(py, m)?;
    QiniuInvalidStatusCodeError::register(py, m)?;
    QiniuInvalidMethodError::register(py, m)?;
    QiniuInvalidHeaderNameError::register(py, m)?;
    QiniuInvalidHeaderValueError::register(py, m)?;
    QiniuHeaderValueEncodingError::register(py, m)?;
    QiniuInvalidIpAddrError::register(py, m)?;
    QiniuInvalidEndpointError::register(py, m)?;
    QiniuJsonError::register(py, m)?;
    QiniuTimeError::register(py, m)?;
    QiniuIoError::register(py, m)?;
    QiniuUploadTokenFormatError::register(py, m)?;
    QiniuBase64Error::register(py, m)?;
    QiniuMimeParseError::register(py, m)?;
    QiniuCallbackError::register(py, m)?;
    QiniuHttpCallError::register(py, m)?;
    QiniuIsahcError::register(py, m)?;
    QiniuTrustDNSError::register(py, m)?;
    QiniuInvalidDomainWithPortError::register(py, m)?;
    QiniuInvalidIpAddrWithPortError::register(py, m)?;
    QiniuApiCallError::register(py, m)?;
    QiniuDownloadError::register(py, m)?;
    QiniuAuthorizationError::register(py, m)?;
    QiniuInvalidPrefixLengthError::register(py, m)?;
    Ok(())
}

macro_rules! create_exception_with_info {
    ($module: ident, $name: ident, $name_str: literal, $base: ty, $inner_name: ident, $inner_type:ty, $doc: expr) => {
        create_exception!($module, $name, $base, $doc);

        #[pyclass]
        #[derive(Clone)]
        pub(super) struct $inner_name(std::sync::Arc<$inner_type>);

        #[pymethods]
        impl $inner_name {
            fn __repr__(&self) -> String {
                format!("{:?}", self.0)
            }

            fn __str__(&self) -> String {
                format!("{}", self.0)
            }
        }

        impl From<$inner_type> for $inner_name {
            fn from(t: $inner_type) -> $inner_name {
                $inner_name(std::sync::Arc::new(t))
            }
        }

        impl AsRef<$inner_type> for $inner_name {
            fn as_ref(&self) -> &$inner_type {
                &self.0
            }
        }

        impl $name {
            fn register(py: Python<'_>, m: &PyModule) -> PyResult<()> {
                m.add($name_str, py.get_type::<$name>())?;
                m.add_class::<$inner_name>()?;
                Ok(())
            }

            #[allow(dead_code)]
            pub(super) fn from_err(err: $inner_type) -> PyErr {
                Self::new_err($inner_name::from(err))
            }
        }
    };
}

create_exception!(
    qiniu_sdk_bindings,
    QiniuUserAgentInitializeError,
    PyRuntimeError,
    "?????????????????????????????????"
);
create_exception!(
    qiniu_sdk_bindings,
    QiniuInvalidPortError,
    PyValueError,
    "???????????????????????????"
);
create_exception!(
    qiniu_sdk_bindings,
    QiniuBodySizeMissingError,
    PyTypeError,
    "???????????? body_len ????????????"
);
create_exception!(
    qiniu_sdk_bindings,
    QiniuEmptyChainCredentialsProvider,
    PyValueError,
    "????????? ChainCredentialsProvider ??????"
);
create_exception!(
    qiniu_sdk_bindings,
    QiniuEmptyRegionsProvider,
    PyValueError,
    "????????? StaticRegionsProvider ??????"
);
create_exception!(
    qiniu_sdk_bindings,
    QiniuEmptyEndpoints,
    PyValueError,
    "????????? Endpoints ??????"
);
create_exception!(
    qiniu_sdk_bindings,
    QiniuEmptyChainedResolver,
    PyValueError,
    "????????? ChainedResolver ??????"
);
create_exception!(
    qiniu_sdk_bindings,
    QiniuUnsupportedTypeError,
    PyValueError,
    "??????????????????????????????"
);
create_exception!(
    qiniu_sdk_bindings,
    QiniuInvalidConcurrency,
    PyValueError,
    "?????????????????????"
);
create_exception!(
    qiniu_sdk_bindings,
    QiniuInvalidObjectSize,
    PyValueError,
    "????????????????????????"
);
create_exception!(
    qiniu_sdk_bindings,
    QiniuInvalidPartSize,
    PyValueError,
    "????????????????????????"
);
create_exception!(
    qiniu_sdk_bindings,
    QiniuInvalidMultiply,
    PyValueError,
    "????????????????????????"
);
create_exception!(
    qiniu_sdk_bindings,
    QiniuInvalidLimitation,
    PyValueError,
    "????????????????????????"
);
create_exception!(
    qiniu_sdk_bindings,
    QiniuInvalidSourceKeyLengthError,
    PyValueError,
    "??????????????? KEY ????????????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuCallbackError,
    "QiniuCallbackError",
    PyRuntimeError,
    QiniuCallbackErrorInfo,
    anyhow::Error,
    "??????????????????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuIsahcError,
    "QiniuIsahcError",
    PyRuntimeError,
    QiniuIsahcErrorInfo,
    qiniu_sdk::isahc::isahc::Error,
    "?????? Isahc ??????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuTrustDNSError,
    "QiniuTrustDNSError",
    PyRuntimeError,
    QiniuTrustDNSErrorKind,
    qiniu_sdk::http_client::trust_dns_resolver::error::ResolveError,
    "?????? Isahc ??????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuInvalidURLError,
    "QiniuInvalidURLError",
    PyValueError,
    QiniuInvalidURLErrorInfo,
    qiniu_sdk::http::uri::InvalidUri,
    "???????????? URL ??????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuInvalidStatusCodeError,
    "QiniuInvalidStatusCodeError",
    PyValueError,
    QiniuInvalidStatusCodeErrorInfo,
    qiniu_sdk::http::InvalidStatusCode,
    "???????????? HTTP ???????????????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuInvalidMethodError,
    "QiniuInvalidMethodError",
    PyValueError,
    QiniuInvalidMethodErrorInfo,
    qiniu_sdk::http::InvalidMethod,
    "???????????? HTTP ????????????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuInvalidHeaderNameError,
    "QiniuInvalidHeaderNameError",
    PyValueError,
    QiniuInvalidHeaderNameErrorInfo,
    qiniu_sdk::http::InvalidHeaderName,
    "???????????? HTTP ???????????????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuInvalidHeaderValueError,
    "QiniuInvalidHeaderValueError",
    PyValueError,
    QiniuInvalidHeaderValueErrorInfo,
    qiniu_sdk::http::InvalidHeaderValue,
    "???????????? HTTP ????????????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuHeaderValueEncodingError,
    "QiniuHeaderValueEncodingError",
    PyValueError,
    QiniuHeaderValueEncodingErrorInfo,
    qiniu_sdk::http::header::ToStrError,
    "?????? HTTP ???????????????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuInvalidIpAddrError,
    "QiniuInvalidIpAddrError",
    PyValueError,
    QiniuInvalidIpAddrErrorInfo,
    std::net::AddrParseError,
    "???????????? IP ????????????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuInvalidDomainWithPortError,
    "QiniuInvalidDomainWithPortError",
    PyValueError,
    QiniuInvalidDomainWithPortErrorInfo,
    qiniu_sdk::http_client::DomainWithPortParseError,
    "????????????????????????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuInvalidIpAddrWithPortError,
    "QiniuInvalidIpAddrWithPortError",
    PyValueError,
    QiniuInvalidIpAddrWithPortErrorInfo,
    qiniu_sdk::http_client::IpAddrWithPortParseError,
    "???????????? IP ????????????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuInvalidEndpointError,
    "QiniuInvalidEndpointError",
    PyValueError,
    QiniuInvalidEndpointErrorInfo,
    qiniu_sdk::http_client::EndpointParseError,
    "??????????????????????????????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuJsonError,
    "QiniuJsonError",
    PyValueError,
    QiniuJsonErrorInfo,
    serde_json::Error,
    "?????? JSON ??????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuTimeError,
    "QiniuTimeError",
    PyValueError,
    QiniuTimeErrorInfo,
    SystemTimeError,
    "??????????????????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuBase64Error,
    "QiniuBase64Error",
    PyValueError,
    QiniuBase64ErrorInfo,
    qiniu_sdk::utils::base64::DecodeError,
    "?????? Base64 ????????????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuMimeParseError,
    "QiniuMimeParseError",
    PyValueError,
    QiniuMimeParseErrorInfo,
    qiniu_sdk::http_client::mime::FromStrError,
    "?????? MIME ????????????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuUploadTokenFormatError,
    "QiniuUploadTokenFormatError",
    PyValueError,
    QiniuUploadTokenFormatErrorInfo,
    qiniu_sdk::upload_token::ParseError,
    "??????????????????????????????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuIoError,
    "QiniuIoError",
    PyIOError,
    QiniuIoErrorInfo,
    IoError,
    "???????????? IO ??????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuHttpCallError,
    "QiniuHttpCallError",
    PyIOError,
    QiniuHttpCallErrorInfo,
    qiniu_sdk::http::ResponseError,
    "?????? HTTP ????????????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuApiCallError,
    "QiniuApiCallError",
    PyIOError,
    QiniuApiCallErrorInfo,
    MaybeOwned<'static, qiniu_sdk::http_client::ResponseError>,
    "?????? API ????????????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuDownloadError,
    "QiniuDownloadError",
    PyIOError,
    QiniuDownloadErrorInfo,
    qiniu_sdk::download::DownloadError,
    "??????????????????"
);
create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuAuthorizationError,
    "QiniuAuthorizationError",
    PyIOError,
    QiniuAuthorizationErrorInfo,
    qiniu_sdk::http_client::AuthorizationError,
    "??????????????????"
);

create_exception_with_info!(
    qiniu_sdk_bindings,
    QiniuInvalidPrefixLengthError,
    "QiniuInvalidPrefixLengthError",
    PyValueError,
    QiniuInvalidPrefixLengthErrorInfo,
    qiniu_sdk::http_client::PrefixLenError,
    "????????????????????????????????????"
);
