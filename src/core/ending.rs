use std::collections::HashMap;
use std::io::sink;
use keride::cesr::{Cigar, Indexer, Siger};
use keride::signing::Signer;
use reqwest::header::HeaderMap;
use sfv::{BareItem, Dictionary, InnerList, Item, List, ListEntry, Parameters, SerializeValue};
use chrono::{DateTime, Utc};
use serde_json::to_string;

pub struct Signage {
    markers: HashMap<String, Siger>,
    indexed: bool,
    signer: Option<String>,
    ordinal: Option<String>,
    digest: Option<String>,
    kind: Option<String>,
}

pub fn signature(signanges: Vec<Signage>) -> HashMap<String, String> {
    let mut values: Vec<String> = Vec::new();

    for signage in signanges {
        let mut tags: Vec<String> = Vec::new();
        let mut markers: Vec<Siger> = Vec::new();

        for (key, value) in &signage.markers {
            tags.push(key.to_string());
            markers.push(value.clone())
        }

        let mut items: Vec<String> = Vec::new();
        let tag = String::from("indexed");

        let val = if signage.indexed {
            String::from("?1")
        } else {
            String::from("?0")
        };

        items.push(format!("{}={}", tag, val));
        if let Some(signer) = signage.signer {
            items.push(format!("signer={}", signer))
        }

        if let Some(ordinal) = signage.ordinal {
            items.push(format!("ordinal={}", ordinal))
        }

        if let Some(digest) = signage.digest {
            items.push(format!("digest={}", digest))
        }

        if let Some(kind) = signage.kind {
            items.push(format!("kind={}", kind))
        }

        for (i, &ref marker) in markers.iter().enumerate() {
            let tag = &tags[i];
            let val = marker.qb64().unwrap();

            items.push(format!("{}={}", tag, String::from(val)))
        }

        values.push(items.join(";"))
    }

    let mut out = HashMap::new();
    out.insert(String::from("Signature"), values.join(","));
    return out;
}

pub fn siginput(
    name: String,
    method: String,
    path: String,
    headers: &HashMap<String, String>,
    signers: &Vec<Signer>,
    fields: &Vec<String>,
    date: Option<DateTime<Utc>>,
    expires: Option<String>,
    nonce: Option<String>,
    keyid: Option<String>,
    context: Option<String>,
    alg: Option<String>,
) -> (String, Cigar) {
    let mut sid = Dictionary::new();
    let mut items = Vec::new();
    let mut ifields = Vec::new();
    let mut field_names = Vec::new();
    let mut values = Vec::new();

    for field in fields {
        if field.starts_with("@") {
            if field.eq("@method") {
                items.push(format!("{}={}", field, method));
                ifields.push(Item::new(BareItem::String(field.to_string())));
                field_names.push(field.to_string());
            } else if field.eq("@path") {
                items.push(format!("{}={}", field, path));
                ifields.push(Item::new(BareItem::String(field.to_string())));
                field_names.push(field.to_string());
            }
        } else {
            let f = field.to_lowercase();
            if !headers.contains_key(&f) {
                continue;
            }

            ifields.push(Item::new(BareItem::String(f.clone())));
            field_names.push(field.to_string());
            let value = headers.get(&f).unwrap();
            items.push(format!("{}={}", f, value))
        }
    }

    let created = if let Some(date) = date {
        date.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
    } else {
        Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
    };

    values.push(format!("({})", field_names.join(" ")));
    values.push(format!("created={}", created));

    let mut field_list_params = Parameters::new();
    field_list_params.insert("created".into(), BareItem::String(created));

    if let Some(expires) = expires {
        values.push(format!("expires={}", expires));
        field_list_params.insert("expires".into(), BareItem::String(expires));
    }
    if let Some(nonce) = nonce {
        values.push(format!("nonce={}", nonce));
        field_list_params.insert("nonce".into(), BareItem::String(nonce));
    }
    if let Some(keyid) = keyid {
        values.push(format!("keyid={}", keyid));
        field_list_params.insert("keyid".into(), BareItem::String(keyid));
    }
    if let Some(context) = context {
        values.push(format!("context={}", context));
        field_list_params.insert("context".into(), BareItem::String(context));
    }
    if let Some(alg) = alg {
        values.push(format!("alg={}", alg));
        field_list_params.insert("alg".into(), BareItem::String(alg));
    }

    let params = values.join(";");
    items.push(format!("@signature-params: {params}"));
    let ser = items.join("\n");
    let mut sigers = Vec::new();

    for signer in signers {
        sigers.push(signer.sign_unindexed(ser.as_bytes()).unwrap())
    }

    let mut sid = Dictionary::new();
    let field_list = InnerList::with_params(ifields, field_list_params);

    sid.insert(name.into(), ListEntry::from(field_list));

    return (format!("{}", sid.serialize_value().unwrap()), sigers.get(0).unwrap().clone());
}


mod test {
    use std::collections::HashMap;
    use chrono::{TimeZone, Utc};
    use keride::cesr::{Matter, Salter};
    use keride::signify::creating::SaltyCreator;
    use keride::signing::Signer;
    use crate::core::ending;
    use crate::core::ending::{siginput, Signage};

    #[test]
    fn siginput_basic() {
        let salt = Salter::new_with_raw(b"0123456789abcdef", None, None).unwrap().qb64().unwrap();
        let creator = SaltyCreator::new(Some(salt.as_str()), None, None, None).unwrap();
        let signers = creator.create(None, None, None, None, None, None, None, None, false);

        let headers = HashMap::new();
        let fields = vec![String::from("@method"), String::from("@path")];

        let date = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap();
        let val = siginput(String::from("signify"),
                           String::from("POST"),
                           String::from("/boot"),
                           &headers, &signers, &fields, None,None, None, None, None, None, );
        let val = siginput(String::from("signify"),
                           String::from("POST"),
                           String::from("/boot"),
                           &headers, &signers, &fields,
                           Some(date),
                           Some(String::from("2023-05-30T20:25:50.628Z")),
                           None,
                           Some(String::from("BA89hKezugU2LFKiFVbitoHAxXqJh6HQ8Rn9tH7fxd68"))
                           , None, None, );

        assert_eq!(val.0, "signify=(\"@method\" \"@path\");created=\"2014-07-08T09:10:11.000Z\";expires=\"2023-05-30T20:25:50.628Z\";keyid=\"BA89hKezugU2LFKiFVbitoHAxXqJh6HQ8Rn9tH7fxd68\"");
        assert_eq!(val.1.qb64().unwrap(), "0BCj96uR0k1P-WUtA3iUfb985WgM3JSMnAar_pI8ropFYNmBMgepVxwlqbi4JCnPi0voBJSA__9Es3Nhk4giHGkI");

        let fields = vec![String::from("@method"), String::from("@path"), String::from("Signify-Resource"), String::from("Signify-Timestamp")];
        let headers = HashMap::from([
            (String::from("signify-resource"), String::from("BA89hKezugU2LFKiFVbitoHAxXqJh6HQ8Rn9tH7fxd68")),
            (String::from("signify-timestamp"), String::from("2023-05-30T20:25:50.628Z"))
        ]);
        let val = siginput(String::from("signify"),
                           String::from("POST"),
                           String::from("/boot"),
                           &headers, &signers, &fields, Some(date),None,
                           Some(String::from("7ORBtM9xnEUiBSWL1pHBJTit")),
                           None,
                           Some(String::from("KERI")), Some(String::from("Ed25519")));

        assert_eq!(val.0, "signify=(\"@method\" \"@path\" \"signify-resource\" \"signify-timestamp\");created=\"2014-07-08T09:10:11.000Z\";nonce=\"7ORBtM9xnEUiBSWL1pHBJTit\";context=\"KERI\";alg=\"Ed25519\"");
        assert_eq!(val.1.qb64().unwrap(), "0BAiX2XCBCTGdSjNxwkkI9CgKdy41X6LDKFHkRRE69g58wlNXvfZaIpQUHwyxVWAolXerUlXmng4FTZmDgGtCJ8O");
    }

    #[test]
    fn signature_indexed() {
        let text = b"{\"seid\":\"BA89hKezugU2LFKiFVbitoHAxXqJh6HQ8Rn9tH7fxd68\",\"name\":\"wit0\",\
        \"dts\":\"2021-01-01T00:00:00.000000+00:00\",\"scheme\":\"http\",\"host\":\"localhost\",\
        \"port\":8080,\"path\":\"/witness\"}";

        let salt = Salter::new_with_raw(b"0123456789abcdef", None, None).unwrap().qb64().unwrap();
        let creator = SaltyCreator::new(Some(salt.as_str()), None, None, None).unwrap();
        let signers = creator.create(None, None, None, None, None, None, None, None, false);
        let signer = &signers[0];
        let siger = signer.sign_indexed(text, false, 0, None).unwrap();

        let mut markers = HashMap::new();
        markers.insert(String::from("signify"), siger);

        let signage = Signage {
            markers,
            indexed: true,
            signer: None,
            ordinal: None,
            digest: None,
            kind: None,
        };

        let mut signages = Vec::new();
        signages.push(signage);
        let headers = ending::signature(signages);
        assert_eq!(headers.get("Signature").unwrap(), "indexed=?1;signify=AACJ6cKLj7ORBtM9xnEUiBSWL1pHBJTit622dUb42eIkkrAJpgWaYkjk8-6NhBGCdYuWvSoQBMvNM-6vmEheSR0B");
    }

    #[test]
    fn signature_unindexed() {
        let text = b"{\"seid\":\"BA89hKezugU2LFKiFVbitoHAxXqJh6HQ8Rn9tH7fxd68\",\"name\":\"wit0\",\
        \"dts\":\"2021-01-01T00:00:00.000000+00:00\",\"scheme\":\"http\",\"host\":\"localhost\",\
        \"port\":8080,\"path\":\"/witness\"}";

        let salt = Salter::new_with_raw(b"0123456789abcdef", None, None).unwrap().qb64().unwrap();
        let creator = SaltyCreator::new(Some(salt.as_str()), None, None, None).unwrap();
        let signers = creator.create(None, None, None, None, None, None, None, None, false);
        let signer = &signers[0];
        let siger = signer.sign_indexed(text, false, 0, None).unwrap();

        let mut markers = HashMap::new();
        markers.insert(String::from("signify"), siger);

        let signage = Signage {
            markers,
            indexed: false,
            signer: None,
            ordinal: None,
            digest: None,
            kind: None,
        };

        let mut signages = Vec::new();
        signages.push(signage);
        let headers = ending::signature(signages);
        assert_eq!(headers.get("Signature").unwrap(), "indexed=?0;signify=AACJ6cKLj7ORBtM9xnEUiBSWL1pHBJTit622dUb42eIkkrAJpgWaYkjk8-6NhBGCdYuWvSoQBMvNM-6vmEheSR0B");
    }

    #[test]
    fn signature_signer() {
        let text = b"{\"seid\":\"BA89hKezugU2LFKiFVbitoHAxXqJh6HQ8Rn9tH7fxd68\",\"name\":\"wit0\",\
        \"dts\":\"2021-01-01T00:00:00.000000+00:00\",\"scheme\":\"http\",\"host\":\"localhost\",\
        \"port\":8080,\"path\":\"/witness\"}";

        let salt = Salter::new_with_raw(b"0123456789abcdef", None, None).unwrap().qb64().unwrap();
        let creator = SaltyCreator::new(Some(salt.as_str()), None, None, None).unwrap();
        let signers = creator.create(None, None, None, None, None, None, None, None, false);
        let signer = &signers[0];
        let siger = signer.sign_indexed(text, false, 0, None).unwrap();

        let mut markers = HashMap::new();
        markers.insert(String::from("signify"), siger);

        let signage = Signage {
            markers,
            indexed: true,
            signer: Some(signer.verfer().qb64().unwrap()),
            ordinal: None,
            digest: None,
            kind: None,
        };

        let mut signages = Vec::new();
        signages.push(signage);
        let headers = ending::signature(signages);
        assert_eq!(headers.get("Signature").unwrap(),
                   "indexed=?1;signer=DMZy6qbgnKzvCE594tQ4SPs6pIECXTYQBH7BkC4hNY3E;signify=AACJ6cKLj7ORBtM9xnEUiBSWL1pHBJTit622dUb42eIkkrAJpgWaYkjk8-6NhBGCdYuWvSoQBMvNM-6vmEheSR0B");
    }
}