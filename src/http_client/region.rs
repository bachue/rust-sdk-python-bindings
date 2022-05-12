use crate::{
    credential::CredentialProvider,
    exceptions::{
        QiniuApiCallError, QiniuEmptyRegionsProvider, QiniuInvalidDomainWithPortError,
        QiniuInvalidEndpointError, QiniuInvalidIpAddrWithPortError, QiniuInvalidServiceNameError,
    },
    utils::extract_endpoints,
};
use pyo3::{prelude::*, pyclass::CompareOp};
use qiniu_sdk::http_client::{
    DomainWithPortParseError, EndpointParseError, EndpointsGetOptions, IpAddrWithPortParseError,
};
use std::{path::PathBuf, time::Duration};

pub(super) fn register(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<DomainWithPort>()?;
    m.add_class::<IpAddrWithPort>()?;
    m.add_class::<Endpoint>()?;
    m.add_class::<ServiceName>()?;
    m.add_class::<Endpoints>()?;
    m.add_class::<EndpointsProvider>()?;
    m.add_class::<Region>()?;
    m.add_class::<RegionsProvider>()?;
    m.add_class::<AllRegionsProvider>()?;

    Ok(())
}

/// 域名和端口号
///
/// 用来表示一个七牛服务器的地址，端口号是可选的，如果不提供，则根据传输协议判定默认的端口号。
#[pyclass]
#[pyo3(text_signature = "(domain, port = None)")]
#[derive(Clone)]
struct DomainWithPort(qiniu_sdk::http_client::DomainWithPort);

#[pymethods]
impl DomainWithPort {
    #[new]
    #[args(port = "None")]
    fn new(domain: String, port: Option<u16>) -> PyResult<Self> {
        let host = if let Some(port) = port {
            format!("{}:{}", domain, port).parse()
        } else {
            domain.parse()
        }
        .map_err(|err: DomainWithPortParseError| {
            QiniuInvalidDomainWithPortError::new_err(err.to_string())
        })?;
        Ok(Self(host))
    }

    /// 获取域名
    #[getter]
    fn get_domain(&self) -> &str {
        self.0.domain()
    }

    /// 获取端口
    #[getter]
    fn get_port(&self) -> Option<u16> {
        self.0.port().map(|port| port.get())
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.0 == other.0).to_object(py),
            _ => py.NotImplemented(),
        }
    }
}

/// IP 地址和端口号
///
/// 用来表示一个七牛服务器的地址，端口号是可选的，如果不提供，则根据传输协议判定默认的端口号。
#[pyclass]
#[pyo3(text_signature = "(ip, port = None)")]
#[derive(Clone)]
struct IpAddrWithPort(qiniu_sdk::http_client::IpAddrWithPort);

#[pymethods]
impl IpAddrWithPort {
    #[new]
    #[args(port = "None")]
    fn new(ip_addr: String, port: Option<u16>) -> PyResult<Self> {
        let host = if let Some(port) = port {
            format!("{}:{}", ip_addr, port).parse()
        } else {
            ip_addr.parse()
        }
        .map_err(|err: IpAddrWithPortParseError| {
            QiniuInvalidIpAddrWithPortError::new_err(err.to_string())
        })?;
        Ok(Self(host))
    }

    /// 获取 IP 地址
    #[getter]
    fn get_ip_addr(&self) -> String {
        self.0.ip_addr().to_string()
    }

    /// 获取端口
    #[getter]
    fn get_port(&self) -> Option<u16> {
        self.0.port().map(|port| port.get())
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.0 == other.0).to_object(py),
            _ => py.NotImplemented(),
        }
    }
}

/// 终端地址
///
/// 用来表示一个域名和端口号，或 IP 地址和端口号。
#[pyclass]
#[pyo3(text_signature = "(domain_or_ip_addr, port = None)")]
#[derive(Clone)]
pub(crate) struct Endpoint(qiniu_sdk::http_client::Endpoint);

#[pymethods]
impl Endpoint {
    #[new]
    #[args(port = "None")]
    fn new(domain_or_ip_addr: String, port: Option<u16>) -> PyResult<Self> {
        let host = if let Some(port) = port {
            format!("{}:{}", domain_or_ip_addr, port).parse()
        } else {
            domain_or_ip_addr.parse()
        }
        .map_err(|err: EndpointParseError| QiniuInvalidEndpointError::new_err(err.to_string()))?;
        Ok(Self(host))
    }

    /// 获取域名
    #[getter]
    fn get_domain(&self) -> Option<&str> {
        self.0.domain()
    }

    /// 获取 IP 地址
    #[getter]
    fn get_ip_addr(&self) -> Option<String> {
        self.0.ip_addr().map(|ip| ip.to_string())
    }

    /// 获取端口
    #[getter]
    fn get_port(&self) -> Option<u16> {
        self.0.port().map(|port| port.get())
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.0 == other.0).to_object(py),
            _ => py.NotImplemented(),
        }
    }
}

impl Endpoint {
    pub(crate) fn into_inner(self) -> qiniu_sdk::http_client::Endpoint {
        self.0
    }
}

#[pyclass]
#[derive(Clone, Copy)]
enum ServiceName {
    /// 上传服务
    Up = 0,

    /// 下载服务
    Io = 1,

    /// 存储空间管理服务
    Uc = 2,

    /// 元数据管理服务
    Rs = 3,

    /// 元数据列举服务
    Rsf = 4,

    /// API 入口服务
    Api = 5,

    /// S3 入口服务
    S3 = 6,
}

impl From<ServiceName> for qiniu_sdk::http_client::ServiceName {
    fn from(svc: ServiceName) -> Self {
        match svc {
            ServiceName::Up => qiniu_sdk::http_client::ServiceName::Up,
            ServiceName::Io => qiniu_sdk::http_client::ServiceName::Io,
            ServiceName::Uc => qiniu_sdk::http_client::ServiceName::Uc,
            ServiceName::Rs => qiniu_sdk::http_client::ServiceName::Rs,
            ServiceName::Rsf => qiniu_sdk::http_client::ServiceName::Rsf,
            ServiceName::Api => qiniu_sdk::http_client::ServiceName::Api,
            ServiceName::S3 => qiniu_sdk::http_client::ServiceName::S3,
        }
    }
}

impl TryFrom<qiniu_sdk::http_client::ServiceName> for ServiceName {
    type Error = PyErr;

    fn try_from(svc: qiniu_sdk::http_client::ServiceName) -> Result<Self, Self::Error> {
        match svc {
            qiniu_sdk::http_client::ServiceName::Up => Ok(ServiceName::Up),
            qiniu_sdk::http_client::ServiceName::Io => Ok(ServiceName::Io),
            qiniu_sdk::http_client::ServiceName::Uc => Ok(ServiceName::Uc),
            qiniu_sdk::http_client::ServiceName::Rs => Ok(ServiceName::Rs),
            qiniu_sdk::http_client::ServiceName::Rsf => Ok(ServiceName::Rsf),
            qiniu_sdk::http_client::ServiceName::Api => Ok(ServiceName::Api),
            qiniu_sdk::http_client::ServiceName::S3 => Ok(ServiceName::S3),
            _ => Err(QiniuInvalidServiceNameError::new_err(format!(
                "Unrecognized ServiceName {:?}",
                svc
            ))),
        }
    }
}

/// 终端地址列表获取接口
///
/// 同时提供阻塞获取接口和异步获取接口，异步获取接口则需要启用 `async` 功能
#[pyclass(subclass)]
#[derive(Clone)]
#[pyo3(text_signature = "(regions_provider)")]
struct EndpointsProvider(Box<dyn qiniu_sdk::http_client::EndpointsProvider>);

#[pymethods]
impl EndpointsProvider {
    #[new]
    fn new(regions_provider: RegionsProvider) -> Self {
        Self(Box::new(
            qiniu_sdk::http_client::RegionsProviderEndpoints::new(regions_provider.0),
        ))
    }

    #[pyo3(text_signature = "(/, service_names = None)")]
    fn get_endpoints(
        &self,
        service_names: Option<Vec<ServiceName>>,
        py: Python<'_>,
    ) -> PyResult<Py<Endpoints>> {
        let service_names = service_names
            .unwrap_or_default()
            .into_iter()
            .map(|svc| svc.into())
            .collect::<Vec<_>>();
        let opts = EndpointsGetOptions::builder()
            .service_names(&service_names)
            .build();
        let endpoints = py
            .allow_threads(|| self.0.get_endpoints(opts))
            .map_err(|err| QiniuApiCallError::new_err(err.to_string()))?
            .into_owned();
        Self::make_initializer(endpoints, py)
    }

    #[pyo3(text_signature = "(/, service_names = None)")]
    fn async_get_endpoints<'p>(
        &self,
        service_names: Option<Vec<ServiceName>>,
        py: Python<'p>,
    ) -> PyResult<&'p PyAny> {
        let provider = self.0.to_owned();
        pyo3_asyncio::async_std::future_into_py(py, async move {
            let service_names = service_names
                .unwrap_or_default()
                .into_iter()
                .map(|svc| svc.into())
                .collect::<Vec<_>>();
            let opts = EndpointsGetOptions::builder()
                .service_names(&service_names)
                .build();
            let endpoints = provider
                .async_get_endpoints(opts)
                .await
                .map_err(|err| QiniuApiCallError::new_err(err.to_string()))?
                .into_owned();
            Python::with_gil(|py| Self::make_initializer(endpoints, py))
        })
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl EndpointsProvider {
    fn make_initializer(
        endpoint: qiniu_sdk::http_client::Endpoints,
        py: Python<'_>,
    ) -> PyResult<Py<Endpoints>> {
        Py::new(
            py,
            (
                Endpoints(endpoint.to_owned()),
                EndpointsProvider(Box::new(endpoint)),
            ),
        )
    }
}

/// 终端地址列表
///
/// 存储一个七牛服务的多个终端地址，包含主要地址列表和备选地址列表
#[pyclass(extends = EndpointsProvider)]
#[pyo3(text_signature = "(preferred_endpoints, alternative_endpoints = None)")]
#[derive(Clone)]
struct Endpoints(qiniu_sdk::http_client::Endpoints);

#[pymethods]
impl Endpoints {
    #[new]
    #[args(alternative_endpoints = "None")]
    fn new(
        preferred_endpoints: Vec<&PyAny>,
        alternative_endpoints: Option<Vec<&PyAny>>,
    ) -> PyResult<(Self, EndpointsProvider)> {
        let mut builder = qiniu_sdk::http_client::EndpointsBuilder::default();
        builder.add_preferred_endpoints(extract_endpoints(preferred_endpoints)?);
        if let Some(alternative_endpoints) = alternative_endpoints {
            builder.add_alternative_endpoints(extract_endpoints(alternative_endpoints)?);
        }
        let endpoints = builder.build();
        Ok((
            Self(endpoints.to_owned()),
            EndpointsProvider(Box::new(endpoints)),
        ))
    }

    /// 返回主要终端地址列表
    #[getter]
    fn get_preferred(&self) -> Vec<Endpoint> {
        self.0.preferred().iter().cloned().map(Endpoint).collect()
    }

    /// 返回备选终端地址列表
    #[getter]
    fn get_alternative(&self) -> Vec<Endpoint> {
        self.0.alternative().iter().cloned().map(Endpoint).collect()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.0 == other.0).to_object(py),
            _ => py.NotImplemented(),
        }
    }
}

/// 区域信息获取接口
///
/// 可以获取一个区域也可以获取多个区域
///
/// 同时提供阻塞获取接口和异步获取接口，异步获取接口则需要启用 `async` 功能
#[pyclass(subclass)]
#[derive(Clone)]
#[pyo3(text_signature = "(regions)")]
struct RegionsProvider(Box<dyn qiniu_sdk::http_client::RegionsProvider>);

#[pymethods]
impl RegionsProvider {
    #[new]
    fn new(regions: Vec<Region>) -> PyResult<Self> {
        let mut iter = regions.into_iter();
        if let Some(region) = iter.next() {
            let mut provider = qiniu_sdk::http_client::StaticRegionsProvider::new(region.0);
            provider.extend(iter.map(|r| r.0));
            Ok(Self(Box::new(provider)))
        } else {
            Err(QiniuEmptyRegionsProvider::new_err("regions is empty"))
        }
    }

    #[pyo3(text_signature = "()")]
    fn get(&self, py: Python<'_>) -> PyResult<Py<Region>> {
        let region = py
            .allow_threads(|| self.0.get(Default::default()))
            .map_err(|err| QiniuApiCallError::new_err(err.to_string()))?
            .into_region();
        Self::make_initializer(region, py)
    }

    #[pyo3(text_signature = "()")]
    fn get_all(&self, py: Python<'_>) -> PyResult<Vec<Py<Region>>> {
        let regions = py
            .allow_threads(|| self.0.get_all(Default::default()))
            .map_err(|err| QiniuApiCallError::new_err(err.to_string()))?
            .into_regions()
            .into_iter()
            .map(|region| Self::make_initializer(region, py))
            .collect::<PyResult<Vec<Py<Region>>>>()?;
        Ok(regions)
    }

    #[pyo3(text_signature = "()")]
    fn async_get<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let provider = self.0.to_owned();
        pyo3_asyncio::async_std::future_into_py(py, async move {
            let region = provider
                .async_get(Default::default())
                .await
                .map_err(|err| QiniuApiCallError::new_err(err.to_string()))?
                .into_region();
            Python::with_gil(|py| Self::make_initializer(region, py))
        })
    }

    #[pyo3(text_signature = "()")]
    fn async_get_all<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let provider = self.0.to_owned();
        pyo3_asyncio::async_std::future_into_py(py, async move {
            let regions = provider
                .async_get_all(Default::default())
                .await
                .map_err(|err| QiniuApiCallError::new_err(err.to_string()))?
                .into_regions()
                .into_iter()
                .map(|region| Python::with_gil(|py| Self::make_initializer(region, py)))
                .collect::<PyResult<Vec<Py<Region>>>>()?;
            Ok(regions)
        })
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl RegionsProvider {
    fn make_initializer(
        region: qiniu_sdk::http_client::Region,
        py: Python<'_>,
    ) -> PyResult<Py<Region>> {
        Py::new(
            py,
            (Region(region.to_owned()), RegionsProvider(Box::new(region))),
        )
    }
}

/// 七牛存储区域
///
/// 提供七牛不同服务的终端地址列表
#[pyclass(extends = RegionsProvider)]
#[pyo3(
    text_signature = "(region_id, /, s3_region_id = None, up_preferred_endpoints = None, up_alternative_endpoints = None, io_preferred_endpoints = None, io_alternative_endpoints = None, uc_preferred_endpoints = None, uc_preferred_endpoints = None, rs_preferred_endpoints = None, rs_alternative_endpoints = None, rsf_preferred_endpoints = None, rsf_alternative_endpoints = None, s3_preferred_endpoints = None, s3_alternative_endpoints = None, api_preferred_endpoints = None, api_alternative_endpoints = None)"
)]
#[derive(Clone)]
struct Region(qiniu_sdk::http_client::Region);

#[pymethods]
impl Region {
    #[new]
    #[args(
        s3_region_id = "None",
        up_preferred_endpoints = "None",
        up_alternative_endpoints = "None",
        io_preferred_endpoints = "None",
        io_alternative_endpoints = "None",
        uc_preferred_endpoints = "None",
        uc_preferred_endpoints = "None",
        rs_preferred_endpoints = "None",
        rs_alternative_endpoints = "None",
        rsf_preferred_endpoints = "None",
        rsf_alternative_endpoints = "None",
        s3_preferred_endpoints = "None",
        s3_alternative_endpoints = "None",
        api_preferred_endpoints = "None",
        api_alternative_endpoints = "None"
    )]
    #[allow(clippy::too_many_arguments)]
    fn new(
        region_id: String,
        s3_region_id: Option<String>,
        up_preferred_endpoints: Option<Vec<&PyAny>>,
        up_alternative_endpoints: Option<Vec<&PyAny>>,
        io_preferred_endpoints: Option<Vec<&PyAny>>,
        io_alternative_endpoints: Option<Vec<&PyAny>>,
        uc_preferred_endpoints: Option<Vec<&PyAny>>,
        uc_alternative_endpoints: Option<Vec<&PyAny>>,
        rs_preferred_endpoints: Option<Vec<&PyAny>>,
        rs_alternative_endpoints: Option<Vec<&PyAny>>,
        rsf_preferred_endpoints: Option<Vec<&PyAny>>,
        rsf_alternative_endpoints: Option<Vec<&PyAny>>,
        s3_preferred_endpoints: Option<Vec<&PyAny>>,
        s3_alternative_endpoints: Option<Vec<&PyAny>>,
        api_preferred_endpoints: Option<Vec<&PyAny>>,
        api_alternative_endpoints: Option<Vec<&PyAny>>,
    ) -> PyResult<(Self, RegionsProvider)> {
        let mut builder = qiniu_sdk::http_client::Region::builder(region_id);
        if let Some(s3_region_id) = s3_region_id {
            builder.s3_region_id(s3_region_id);
        }
        if let Some(endpoints) = up_preferred_endpoints {
            builder.add_up_preferred_endpoints(extract_endpoints(endpoints)?);
        }
        if let Some(endpoints) = up_alternative_endpoints {
            builder.add_up_alternative_endpoints(extract_endpoints(endpoints)?);
        }
        if let Some(endpoints) = io_preferred_endpoints {
            builder.add_io_preferred_endpoints(extract_endpoints(endpoints)?);
        }
        if let Some(endpoints) = io_alternative_endpoints {
            builder.add_io_alternative_endpoints(extract_endpoints(endpoints)?);
        }
        if let Some(endpoints) = uc_preferred_endpoints {
            builder.add_uc_preferred_endpoints(extract_endpoints(endpoints)?);
        }
        if let Some(endpoints) = uc_alternative_endpoints {
            builder.add_uc_alternative_endpoints(extract_endpoints(endpoints)?);
        }
        if let Some(endpoints) = rs_preferred_endpoints {
            builder.add_rs_preferred_endpoints(extract_endpoints(endpoints)?);
        }
        if let Some(endpoints) = rs_alternative_endpoints {
            builder.add_rs_alternative_endpoints(extract_endpoints(endpoints)?);
        }
        if let Some(endpoints) = rsf_preferred_endpoints {
            builder.add_rsf_preferred_endpoints(extract_endpoints(endpoints)?);
        }
        if let Some(endpoints) = rsf_alternative_endpoints {
            builder.add_rsf_alternative_endpoints(extract_endpoints(endpoints)?);
        }
        if let Some(endpoints) = s3_preferred_endpoints {
            builder.add_s3_preferred_endpoints(extract_endpoints(endpoints)?);
        }
        if let Some(endpoints) = s3_alternative_endpoints {
            builder.add_s3_alternative_endpoints(extract_endpoints(endpoints)?);
        }
        if let Some(endpoints) = api_preferred_endpoints {
            builder.add_api_preferred_endpoints(extract_endpoints(endpoints)?);
        }
        if let Some(endpoints) = api_alternative_endpoints {
            builder.add_api_alternative_endpoints(extract_endpoints(endpoints)?);
        }
        let region = builder.build();
        Ok((Self(region.to_owned()), RegionsProvider(Box::new(region))))
    }

    /// 获取区域 ID
    #[getter]
    fn get_region_id(&self) -> &str {
        self.0.region_id()
    }

    /// 获取 S3 区域 ID
    #[getter]
    fn get_s3_region_id(&self) -> &str {
        self.0.s3_region_id()
    }

    /// 获取上传服务终端列表
    #[getter]
    fn get_up(&self) -> PyResult<Py<Endpoints>> {
        encapsulate_endpoints(self.0.up())
    }

    /// 获取上传服务主要终端列表
    #[getter]
    fn get_up_preferred_endpoints(&self) -> Vec<Endpoint> {
        encapsulate_endpoint_vec(self.0.up_preferred_endpoints())
    }

    /// 获取上传服务备选终端列表
    #[getter]
    fn get_up_alternative_endpoints(&self) -> Vec<Endpoint> {
        encapsulate_endpoint_vec(self.0.up_alternative_endpoints())
    }

    /// 获取下载服务终端列表
    #[getter]
    fn get_io(&self) -> PyResult<Py<Endpoints>> {
        encapsulate_endpoints(self.0.io())
    }

    /// 获取下载服务主要终端列表
    #[getter]
    fn get_io_preferred_endpoints(&self) -> Vec<Endpoint> {
        encapsulate_endpoint_vec(self.0.io_preferred_endpoints())
    }

    /// 获取下载服务备选终端列表
    #[getter]
    fn get_io_alternative_endpoints(&self) -> Vec<Endpoint> {
        encapsulate_endpoint_vec(self.0.io_alternative_endpoints())
    }

    /// 获取存储空间管理服务终端列表
    #[getter]
    fn get_uc(&self) -> PyResult<Py<Endpoints>> {
        encapsulate_endpoints(self.0.uc())
    }

    /// 获取存储空间管理服务主要终端列表
    #[getter]
    fn get_uc_preferred_endpoints(&self) -> Vec<Endpoint> {
        encapsulate_endpoint_vec(self.0.uc_preferred_endpoints())
    }

    /// 获取存储空间管理服务备选终端列表
    #[getter]
    fn get_uc_alternative_endpoints(&self) -> Vec<Endpoint> {
        encapsulate_endpoint_vec(self.0.uc_alternative_endpoints())
    }

    /// 获取元数据管理服务终端列表
    #[getter]
    fn get_rs(&self) -> PyResult<Py<Endpoints>> {
        encapsulate_endpoints(self.0.rs())
    }

    /// 获取元数据管理服务主要终端列表
    #[getter]
    fn get_rs_preferred_endpoints(&self) -> Vec<Endpoint> {
        encapsulate_endpoint_vec(self.0.rs_preferred_endpoints())
    }

    /// 获取元数据管理服务备选终端列表
    #[getter]
    fn get_rs_alternative_endpoints(&self) -> Vec<Endpoint> {
        encapsulate_endpoint_vec(self.0.rs_alternative_endpoints())
    }

    /// 获取元数据列举服务终端列表
    #[getter]
    fn get_rsf(&self) -> PyResult<Py<Endpoints>> {
        encapsulate_endpoints(self.0.rsf())
    }

    /// 获取元数据列举服务主要终端列表
    #[getter]
    fn get_rsf_preferred_endpoints(&self) -> Vec<Endpoint> {
        encapsulate_endpoint_vec(self.0.rsf_preferred_endpoints())
    }

    /// 获取元数据列举服务备选终端列表
    #[getter]
    fn get_rsf_alternative_endpoints(&self) -> Vec<Endpoint> {
        encapsulate_endpoint_vec(self.0.rsf_alternative_endpoints())
    }

    /// 获取 API 入口服务终端列表
    #[getter]
    fn get_api(&self) -> PyResult<Py<Endpoints>> {
        encapsulate_endpoints(self.0.api())
    }

    /// 获取 API 入口服务主要终端列表
    #[getter]
    fn get_api_preferred_endpoints(&self) -> Vec<Endpoint> {
        encapsulate_endpoint_vec(self.0.api_preferred_endpoints())
    }

    /// 获取 API 入口服务备选终端列表
    #[getter]
    fn get_api_alternative_endpoints(&self) -> Vec<Endpoint> {
        encapsulate_endpoint_vec(self.0.api_alternative_endpoints())
    }

    /// 获取 S3 入口服务终端列表
    #[getter]
    fn get_s3(&self) -> PyResult<Py<Endpoints>> {
        encapsulate_endpoints(self.0.s3())
    }

    /// 获取 S3 入口服务主要终端列表
    #[getter]
    fn get_s3_preferred_endpoints(&self) -> Vec<Endpoint> {
        encapsulate_endpoint_vec(self.0.s3_preferred_endpoints())
    }

    /// 获取 S3 入口服务备选终端列表
    #[getter]
    fn get_s3_alternative_endpoints(&self) -> Vec<Endpoint> {
        encapsulate_endpoint_vec(self.0.s3_alternative_endpoints())
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.0 == other.0).to_object(py),
            _ => py.NotImplemented(),
        }
    }
}

/// 七牛所有区域信息查询器
#[pyclass(extends = RegionsProvider)]
#[pyo3(
    text_signature = "(credential_provider, /, auto_persistent = True, use_https = False, uc_endpoints = None, cache_lifetime = None, shrink_interval = None)"
)]
#[derive(Clone)]
struct AllRegionsProvider;

#[pymethods]
impl AllRegionsProvider {
    #[new]
    #[args(
        auto_persistent = "true",
        use_https = "false",
        uc_endpoints = "None",
        cache_lifetime = "None",
        shrink_interval = "None"
    )]
    #[allow(clippy::too_many_arguments)]
    fn new(
        credential_provider: CredentialProvider,
        auto_persistent: bool,
        use_https: bool,
        uc_endpoints: Option<Endpoints>,
        cache_lifetime: Option<u64>,
        shrink_interval: Option<u64>,
    ) -> (Self, RegionsProvider) {
        let builder = Self::new_builder(
            credential_provider,
            use_https,
            uc_endpoints,
            cache_lifetime,
            shrink_interval,
        );
        (
            Self,
            RegionsProvider(Box::new(
                builder.default_load_or_create_from(auto_persistent),
            )),
        )
    }

    #[staticmethod]
    #[pyo3(
        text_signature = "(credential_provider, path, /, auto_persistent = True, use_https = False, uc_endpoints = None, cache_lifetime = None, shrink_interval = None)"
    )]
    #[args(
        auto_persistent = "true",
        use_https = "false",
        uc_endpoints = "None",
        cache_lifetime = "None",
        shrink_interval = "None"
    )]
    #[allow(clippy::too_many_arguments)]
    fn load_or_create_from(
        credential_provider: CredentialProvider,
        path: PathBuf,
        auto_persistent: bool,
        use_https: bool,
        uc_endpoints: Option<Endpoints>,
        cache_lifetime: Option<u64>,
        shrink_interval: Option<u64>,
        py: Python<'_>,
    ) -> PyResult<Py<Self>> {
        let builder = Self::new_builder(
            credential_provider,
            use_https,
            uc_endpoints,
            cache_lifetime,
            shrink_interval,
        );
        Py::new(
            py,
            (
                Self,
                RegionsProvider(Box::new(builder.load_or_create_from(path, auto_persistent))),
            ),
        )
    }

    #[staticmethod]
    #[pyo3(
        text_signature = "(credential_provider, /, use_https = False, uc_endpoints = None, cache_lifetime = None, shrink_interval = None)"
    )]
    #[args(
        use_https = "false",
        uc_endpoints = "None",
        cache_lifetime = "None",
        shrink_interval = "None"
    )]
    #[allow(clippy::too_many_arguments)]
    fn in_memory(
        credential_provider: CredentialProvider,
        use_https: bool,
        uc_endpoints: Option<Endpoints>,
        cache_lifetime: Option<u64>,
        shrink_interval: Option<u64>,
        py: Python<'_>,
    ) -> PyResult<Py<Self>> {
        let builder = Self::new_builder(
            credential_provider,
            use_https,
            uc_endpoints,
            cache_lifetime,
            shrink_interval,
        );
        Py::new(py, (Self, RegionsProvider(Box::new(builder.in_memory()))))
    }
}

impl AllRegionsProvider {
    fn new_builder(
        credential_provider: CredentialProvider,
        use_https: bool,
        uc_endpoints: Option<Endpoints>,
        cache_lifetime: Option<u64>,
        shrink_interval: Option<u64>,
    ) -> qiniu_sdk::http_client::AllRegionsProviderBuilder {
        let mut builder =
            qiniu_sdk::http_client::AllRegionsProvider::builder(credential_provider.into_inner());
        builder = builder.use_https(use_https);
        if let Some(uc_endpoints) = uc_endpoints {
            builder = builder.uc_endpoints(uc_endpoints.0);
        }
        if let Some(cache_lifetime) = cache_lifetime {
            builder = builder.cache_lifetime(Duration::from_secs(cache_lifetime));
        }
        if let Some(shrink_interval) = shrink_interval {
            builder = builder.shrink_interval(Duration::from_secs(shrink_interval));
        }
        builder
    }
}

fn encapsulate_endpoint_vec(endpoints: &[qiniu_sdk::http_client::Endpoint]) -> Vec<Endpoint> {
    endpoints.iter().cloned().map(Endpoint).collect()
}

fn encapsulate_endpoints(endpoints: &qiniu_sdk::http_client::Endpoints) -> PyResult<Py<Endpoints>> {
    Python::with_gil(|py| {
        Py::new(
            py,
            (
                Endpoints(endpoints.to_owned()),
                EndpointsProvider(Box::new(endpoints.to_owned())),
            ),
        )
    })
}