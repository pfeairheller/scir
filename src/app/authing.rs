
use anyhow::Ok;

use keride::cesr::common::Tierage;
use keride::cesr::{Diger, matter, Matter, Salter, Serder};
use keride::data::Value;
use keride::error::Result;
use keride::eventing;
use keride::signify::creating::SaltyCreator;
use keride::signing::Signer;


pub struct Controller {
    bran: String,
    tier: String,
    stem: String,
    salter: Salter,
    signer: Signer,
    nsigner: Signer,
    keys: Vec<String>,
    ndigs: Vec<String>,
    pub serder: Serder,
}

impl Controller {
    pub fn new(
        bran: Option<&str>,
        tier: Option<&str>,
        state: Option<&str>
    ) -> Result<Self> {
        let mut controller = Self::default();

        controller.bran = matter::Codex::Salt_128.to_owned() + "A" + bran.unwrap();
        controller.stem = "signify:controller".to_string();
        controller.salter = Salter::new(None, None, None, None, Some(&controller.bran), None).unwrap();
        let mut creator = SaltyCreator::new(Some(&controller.salter.qb64().unwrap()), Some(&controller.stem), Some(&controller.tier), None).unwrap();

        controller.signer = creator.create(None, Some(1), Some(matter::Codex::Ed25519_Seed), Some(0), Some(0), Some(0), None, Some(true), false).pop().unwrap();
        controller.nsigner = creator.create(None, Some(1), Some(matter::Codex::Ed25519_Seed), Some(0), Some(1), Some(0), None, Some(true), false).pop().unwrap();

        controller.keys = vec![];
        controller.keys.push(controller.signer.verfer().qb64().unwrap());
        controller.ndigs = vec![];
        let qb64b: &[u8] = &controller.nsigner.verfer().qb64b().unwrap();
        let diger = Diger::new(Some(qb64b), None, None, None, None, None).unwrap();
        controller.ndigs.push(
            diger.qb64().unwrap()
        );

        controller.serder = controller.derive(state).unwrap();

        Ok(controller)
    }

    fn derive(&self, state: Option<&str>) -> Result<Serder> {
        let sith = "1".to_string();
        let nsith = "1".to_string();
        let mut serder = eventing::incept(
            &self.keys,
            Some(&Value::String(sith)),
            Some(&self.ndigs),
            Some(&Value::String(nsith)),
            Some(0),
            None, // Some(&wits),
            None,
            None,
            None,
            None,
            Some(matter::Codex::Blake3_256),
            None,
            None,
        ).unwrap();

        Ok(serder)
    }
}

impl Default for Controller {
    fn default() -> Self {
        Controller {
            bran: "".to_string(),
            tier: Tierage::low.to_string(),
            stem: "".to_string(),
            salter: Salter::new(Some(Tierage::low), None, None, None, None, None).unwrap(),
            signer: Default::default(),
            nsigner: Default::default(),
            keys: vec![],
            ndigs: vec![],
            serder: Default::default(),
        }
    }
}

