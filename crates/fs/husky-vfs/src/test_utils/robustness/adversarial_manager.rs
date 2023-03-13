use salsa::DisplayWithDb;

use super::*;

pub(super) struct VfsAdversarialManager {
    module: ModulePath,
    path: PathBuf,
    adversarials: Vec<VfsAdversarial>,
    generator: Option<VfsAdversarialGenerator>,
}

impl VfsAdversarialManager {
    pub(super) fn new(module: ModulePath, path: PathBuf) -> Self {
        let adversarials = match std::fs::read_to_string(&path) {
            Ok(text) => serde_json::from_str(&text).expect("should be deserialized"),
            Err(_) => vec![],
        };
        Self {
            module,
            path,
            adversarials,
            generator: VfsAdversarialGenerator::from_env(),
        }
    }

    pub(super) fn run<Db>(mut self, db: &mut Db, f: &(impl Fn(&Db)))
    where
        Db: VfsDb + ?Sized,
    {
        for adversarial in &self.adversarials {
            if adversarial.test(db, self.module, f).is_err() {
                panic!(
                    "failure against adversial {adversarial:?} for module {}",
                    self.module.display(db)
                )
            }
        }
        if let Some(generator) = self.generator {
            match generator.run(db, self.module, f) {
                Ok(_) => (),
                Err(adversarial) => {
                    self.adversarials.push(adversarial);
                    std::fs::create_dir_all(&self.path.parent().unwrap());
                    husky_io_utils::diff_write(
                        &self.path,
                        &serde_json::to_string_pretty(&self.adversarials)
                            .expect("serializing should work"),
                        true,
                    );
                    panic!("found an adversarial")
                }
            }
        }
    }
}
