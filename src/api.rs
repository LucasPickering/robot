use tide::Request;

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    Ok("Hello".into())
}

pub struct Api {
    app: tide::Server<()>,
}

impl Api {
    pub fn new() -> Self {
        let mut app = tide::new();
        app.at("/orders/shoes").post(order_shoes);
        Self { app }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        Ok(self.app.listen("127.0.0.1:8000").await?)
    }
}
