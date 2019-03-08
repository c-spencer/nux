use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct ZfsSettings {
    pool: ZfsPool,
    root: ZfsRoot,
    datasets: Vec<ZfsDataset>,
}

#[derive(Debug, Deserialize, Clone)]
struct ZfsPool {
    ashift: i32,
    name: String,
}

#[derive(Debug, Deserialize, Clone)]
struct ZfsRoot {
    properties: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Clone)]
struct ZfsDataset {
    name: String,
    mount: String,
    properties: Option<HashMap<String, String>>,
}

impl ZfsSettings {
    pub fn zpool_cmd(&self, partition: &str) -> String {
        format!(
            "zpool create -o ashift={ashift} -O mountpoint=none {props} {name} {partition}",
            name = self.pool.name,
            ashift = self.pool.ashift,
            partition = partition,
            props = zfs_properties_string("O", &self.root.properties)
        )
    }

    pub fn dataset_cmds(&self) -> Vec<String> {
        self.datasets
            .iter()
            .flat_map(|dataset| create_dataset(&self.pool.name, dataset))
            .collect()
    }
}

fn create_dataset(root: &str, dataset: &ZfsDataset) -> Vec<String> {
    let create = format!(
        "zfs create {root}/{name} -o mountpoint=legacy {props}",
        root = root,
        name = dataset.name,
        props = zfs_properties_string("o", &dataset.properties)
    );

    let mkdir = format!("mkdir -p {mount}", mount = dataset.mount);

    let mount = format!(
        "mount -t zfs {root}/{name} {mount}",
        root = root,
        name = dataset.name,
        mount = dataset.mount
    );

    vec![create, mkdir, mount]
}

fn zfs_properties_string(prefix: &str, props: &Option<HashMap<String, String>>) -> String {
    props
        .clone()
        .map(|props| {
            props
                .iter()
                .map(|(k, v)| {
                    format!(
                        "-{prefix} {key}={value}",
                        prefix = prefix,
                        key = k,
                        value = v
                    )
                })
                .collect::<Vec<String>>()
                .join(" ")
        })
        .unwrap_or("".to_owned())
}
