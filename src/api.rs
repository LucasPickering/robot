use crate::config::RobotConfig;
use async_std::sync::RwLock;
use std::sync::Arc;
use tide::{listener::ToListener, Body, Request};

/// HTTP API that allows users to read robot state and mutate the robot config.
/// Because of this, the config needs to be wrapped in a read-write lock, so
/// that mutations can be synchronized with reads during the main loop. This API
/// provides NO authentication or authorization!
pub struct Api {
    app: tide::Server<State>,
}

impl Api {
    /// Set up (but do not launch!) the HTTP server
    pub fn new(config: Arc<RwLock<RobotConfig>>) -> Self {
        let mut app = tide::with_state(State { config });
        app.at("/config").get(get_config).post(post_config);
        Self { app }
    }

    /// Launch the HTTP server
    pub async fn run(self) -> anyhow::Result<()> {
        // bruh
        let host = self
            .app
            .state()
            .config
            .read()
            .await
            .api
            .host
            .as_str()
            .to_listener()?;
        Ok(self.app.listen(host).await?)
    }
}

/// API state, accessible to every request
#[derive(Clone, Debug)]
struct State {
    config: Arc<RwLock<RobotConfig>>,
}

/// Read the robot's config
async fn get_config(req: Request<State>) -> tide::Result<Body> {
    Body::from_json(&req.state().config.read().await as &RobotConfig)
}

/// Update the robot's config
async fn post_config(mut req: Request<State>) -> tide::Result<Body> {
    let new_config: RobotConfig = req.body_json().await?;
    *req.state().config.write().await = new_config.clone();
    Body::from_json(&new_config)
}
