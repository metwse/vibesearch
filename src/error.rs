#[cfg(feature = "serde")]
use bincode::error::EncodeError;
use openai_dive::v1::{
    error::APIError,
    resources::chat::{ChatCompletionParametersBuilderError, JsonSchemaBuilderError},
};

#[derive(Debug)]
pub enum Error {
    API(APIError),
    ChatCompletionParametersBuilder(ChatCompletionParametersBuilderError),
    JsonSchemaBuilder(JsonSchemaBuilderError),
    #[cfg(feature = "serde")]
    Encode(EncodeError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::API(error) => write!(f, "api error: {}", error),
            Self::ChatCompletionParametersBuilder(error) => {
                write!(f, "chat completion parameters builder error: {}", error)
            }
            Self::JsonSchemaBuilder(error) => write!(f, "json schema builder error: {}", error),
            #[cfg(feature = "serde")]
            Self::Encode(error) => write!(f, "encode error: {}", error),
        }
    }
}

impl std::error::Error for Error {}

macro_rules! err_impl_from {
    ($($err:expr),*) => {
        paste::paste! {
            $(
                impl From<[<$err Error>]> for Error {
                    fn from(err: [<$err Error>]) -> Self {
                        Self::$err(err)
                    }
                }
            )*
        }
    };
}

err_impl_from!(
    ChatCompletionParametersBuilder,
    JsonSchemaBuilder,
    API
);

#[cfg(feature = "serde")]
err_impl_from!(
    Encode
);
