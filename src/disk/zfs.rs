use super::duct_util::{Cmd, Expr};

use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct ZfsSettings {
    pub pool: ZfsPool,
    pub properties: Option<HashMap<String, String>>,
    pub datasets: Vec<ZfsDataset>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ZfsPool {
    ashift: i32,
    name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ZfsDataset {
    name: String,
    mount: String,
    properties: Option<HashMap<String, String>>,
}

impl ZfsSettings {
    pub fn zpool_cmd(&self, partition: &str) -> Expr {
        Cmd::new("zpool")
            .arg("create")
            .arg("-o")
            .arg(format!("ashift={}", self.pool.ashift))
            .arg("-O")
            .arg("mountpoint=none")
            .args(self.properties.iter().flat_map(|m| {
                m.iter()
                    .flat_map(|(k, v)| vec!["-o".to_owned(), format!("{}={}", k, v)])
            }))
            .arg(&self.pool.name)
            .arg(partition)
            .to_expr()
    }

    pub fn dataset_cmds(&self) -> Vec<Expr> {
        self.datasets
            .iter()
            .flat_map(|dataset| create_dataset(&self.pool.name, dataset))
            .collect()
    }
}

fn create_dataset(root: &str, dataset: &ZfsDataset) -> Vec<Expr> {
    let canonical_name = format!("{}/{}", root, dataset.name);

    let create = Cmd::new("zfs")
        .arg("create")
        .arg(canonical_name.clone())
        .arg("-o")
        .arg("mountpoint=legacy")
        .args(dataset.properties.iter().flat_map(|m| {
            m.iter()
                .flat_map(|(k, v)| vec!["-o".to_owned(), format!("{}={}", k, v)])
        }))
        .to_expr();

    let mkdir = Cmd::new("mkdir").arg("-p").arg(&dataset.mount).to_expr();

    let mount = Cmd::new("mount")
        .arg("-t")
        .arg("zfs")
        .arg(canonical_name)
        .arg(&dataset.mount)
        .to_expr_with_wait(500);

    vec![create, mkdir, mount]
}
