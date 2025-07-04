use crate::Restic;
use crate::errors::ResticError;
use crate::parsing::ResticMessage;
use log::warn;

impl Restic {
    pub(crate) async fn exec_json<P, F>(
        &self,
        arguments: impl IntoIterator<Item = String>,
        mut on_message: F,
    ) -> Result<(), ResticError>
    where
        P: ResticMessage,
        F: FnMut(P),
    {
        self.exec(
            arguments.into_iter().chain(vec!["--json".to_string()]),
            |string, output_type| match P::parse_message(&string) {
                Ok(message) => on_message(message),
                Err(err) => {
                    warn!("Failed to parse {output_type} message '{string}' due to '{err}'")
                }
            },
        )
        .await
    }
}
