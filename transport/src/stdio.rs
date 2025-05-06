pub struct Stdio {
    reader: tokio::io::BufReader<tokio::io::Stdin>,
    writer: tokio::io::Stdout,
}

impl Stdio {
    pub fn new() -> Self {
        let stdin = tokio::io::stdin();
        let reader = tokio::io::BufReader::new(stdin);
        let writer = tokio::io::stdout();
        Stdio { reader, writer }
    }
}

impl Default for Stdio {
    fn default() -> Self {
        Self::new()
    }
}

impl Stdio {
    pub async fn receive(&mut self) -> tokio::io::Result<jsonrpc::BatchableRequest> {
        loop {
            use tokio::io::AsyncBufReadExt;

            let mut line = String::new();
            self.reader.read_line(&mut line).await?;
            if line.ends_with('\n') {
                line.pop();
            }
            tracing::debug!("Received line: {}", line);
            let req = serde_json::from_str(&line);
            match req {
                Ok(req) => {
                    return Ok(req);
                }
                Err(e) => {
                    let res = jsonrpc::Response::<()>::new(
                        None,
                        Err(jsonrpc::ErrorObject {
                            code: e.into(),
                            message: "failed to receive request".into(),
                            data: None,
                        }),
                    )
                    .into();
                    self.send(res).await?;
                }
            }
        }
    }

    pub async fn send(&mut self, res: jsonrpc::BatchableResponse) -> tokio::io::Result<()> {
        use tokio::io::AsyncWriteExt;

        let data = serde_json::to_string(&res).map_err(|_| {
            tokio::io::Error::new(
                tokio::io::ErrorKind::InvalidData,
                "Failed to serialize data",
            )
        })?;
        tracing::debug!("Sending response: {}", data);
        self.writer.write_all(data.as_ref()).await?;
        self.writer.write_all(b"\n").await?;
        self.writer.flush().await
    }
}
