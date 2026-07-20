use self::builder::endpoints::UnsetApi;

pub mod ids;
pub mod builder;

#[cfg(test)]
mod test;

#[repr(transparent)]
pub struct Endpoint(url::Url);

impl Endpoint {
    pub fn builder() -> builder::EndpointBuilder<UnsetApi> {
        builder::EndpointBuilder::new()
    }
}

impl Into<url::Url> for Endpoint {
    fn into(self) -> url::Url {
        self.0
    }
}
