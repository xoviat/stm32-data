use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::f128::consts::E;
use std::hash::Hash;
use std::path::{Path, PathBuf};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::chips::{Chip, ChipGroup};
use crate::pdsc::schema::pin::Pinout;

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub devices: Devices,
}

#[derive(Serialize, Deserialize)]
pub struct Devices {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "family")]
    pub families: Vec<Family>,
}

#[derive(Serialize, Deserialize)]
pub struct Family {
    #[serde(rename = "@Dfamily")]
    pub dfamily: String,
    #[serde(rename = "@Dvendor")]
    pub dvendor: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub processor: Processor,
    pub book: Vec<Book>,
    pub description: String,
    #[serde(rename = "feature")]
    pub features: Vec<Feature>,
    #[serde(rename = "environment")]
    pub environments: Vec<Environment>,
    #[serde(rename = "subFamily")]
    pub sub_families: Vec<SubFamily>,
}

#[derive(Serialize, Deserialize)]
pub struct Processor {
    #[serde(rename = "@Dcore")]
    pub dcore: String,
    #[serde(rename = "@DcoreVersion")]
    pub dcore_version: String,
    #[serde(rename = "@Dfpu")]
    pub dfpu: String,
    #[serde(rename = "@Dmpu")]
    pub dmpu: String,
    #[serde(rename = "@Ddsp")]
    pub ddsp: String,
    #[serde(rename = "@Dtz")]
    pub dtz: String,
    #[serde(rename = "@Dendian")]
    pub dendian: String,
    #[serde(rename = "@Dclock")]
    pub dclock: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubFamily {
    #[serde(rename = "@DsubFamily")]
    pub dsub_family: String,
    pub memory: Memory,
    pub book: Vec<Book>,
    #[serde(rename = "feature")]
    pub features: Vec<Feature>,
    #[serde(rename = "environment")]
    pub environments: Vec<Environment>,
    #[serde(rename = "device")]
    pub devices: Vec<Device>,
}

#[derive(Serialize, Deserialize)]
pub struct Memory {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@access")]
    pub access: String,
    #[serde(rename = "@start")]
    pub start: String,
    #[serde(rename = "@size")]
    pub size: String,
    #[serde(rename = "@uninit")]
    pub uninit: String,
    #[serde(rename = "@default")]
    pub default: String,
    #[serde(rename = "@startup")]
    pub startup: String,
}

#[derive(Serialize, Deserialize)]
pub struct Feature {
    #[serde(rename = "@type")]
    pub feature_type: String,
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@n")]
    pub n: String,
    #[serde(rename = "@m")]
    pub m: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Idcode {
    #[serde(rename = "@address")]
    pub address: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@ap")]
    pub ap: String,
}

/// Corresponds to one `xml::Mcu`
#[derive(Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "@Dname")]
    pub dname: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub compile: Compile,
    pub memory: Vec<Memory>,
    pub algorithm: Algorithm,
    pub book: Vec<Book>,
    pub feature: Vec<Feature>,
    #[serde(rename = "environment")]
    pub environments: Vec<Environment>,
    pub debug: Debug,
    pub flashinfo: Flashinfo,
    #[serde(rename = "variant")]
    pub variants: Vec<Variant>,
}

#[derive(Serialize, Deserialize)]
pub struct Compile {
    #[serde(rename = "@header")]
    pub header: String,
    #[serde(rename = "@define")]
    pub define: String,
}

#[derive(Serialize, Deserialize)]
pub struct Algorithm {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@start")]
    pub start: String,
    #[serde(rename = "@size")]
    pub size: String,
    #[serde(rename = "@RAMstart")]
    pub ramstart: String,
    #[serde(rename = "@RAMsize")]
    pub ramsize: String,
    #[serde(rename = "@default")]
    pub default: String,
}

#[derive(Serialize, Deserialize)]
pub struct Book {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@title")]
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct Environment {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "device")]
    pub device: STDevice,
}

#[derive(Serialize, Deserialize, Default)]
pub struct STDevice {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub descriptors: Descriptors,
    #[serde(rename = "extra-attributes")]
    pub extra_attributes: ExtraAttributes,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Descriptors {
    #[serde(rename = "descriptor")]
    pub descriptors: Vec<Descriptor>,
}

#[derive(Serialize, Deserialize)]
pub struct Descriptor {
    #[serde(rename = "@schemaType")]
    pub schema_type: String,
    #[serde(rename = "@path")]
    pub path: String,
    #[serde(rename = "@schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "@version")]
    pub version: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ExtraAttributes {
    #[serde(rename = "extra-attribute")]
    pub extra_attributes: Vec<ExtraAttribute>,
}

#[derive(Serialize, Deserialize)]
pub struct ExtraAttribute {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Debug {
    #[serde(rename = "@svd")]
    pub svd: String,
    #[serde(rename = "@__ap")]
    pub _ap: String,
}

#[derive(Serialize, Deserialize)]
pub struct Flashinfo {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@start")]
    pub start: String,
    #[serde(rename = "@pagesize")]
    pub pagesize: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub block: Block,
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    #[serde(rename = "@count")]
    pub count: String,
    #[serde(rename = "@size")]
    pub size: String,
}

#[derive(Serialize, Deserialize)]
pub struct Variant {
    #[serde(rename = "@Dvariant")]
    pub dvariant: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "feature")]
    pub features: Vec<Feature>,
    #[serde(rename = "environment")]
    pub environments: Vec<Environment>,
}

mod schema {
    pub mod pin {
        use serde::{Deserialize, Serialize};

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Pinout {
            #[serde(rename = "schema_version")]
            pub schema_version: String,
            #[serde(rename = "characteristics")]
            pub characteristics: Characteristics,
            #[serde(rename = "pin_type_description")]
            pub pin_type_description: PinTypeDescription,
            #[serde(rename = "io_structure_type_description")]
            pub io_structure_type_description: IoStructureTypeDescription,
            #[serde(rename = "io_structure_options_description")]
            pub io_structure_options_description: IoStructureOptionsDescription,
            #[serde(rename = "package_pins")]
            pub package_pins: Vec<String>,
            #[serde(rename = "signals")]
            pub signals: Vec<Signal>,
            #[serde(rename = "bonds")]
            pub bonds: Vec<Bond>,
            #[serde(rename = "version")]
            pub version: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Characteristics {
            #[serde(rename = "package_name")]
            pub package_name: String,
            #[serde(rename = "package_type")]
            pub package_type: String,
            #[serde(rename = "die_name")]
            pub die_name: String,
            #[serde(rename = "NbIOs")]
            pub nb_ios: i64,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct PinTypeDescription {
            pub s: String,
            #[serde(rename = "I/O")]
            pub i_o: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct IoStructureTypeDescription {
            #[serde(rename = "RST")]
            pub rst: String,
            #[serde(rename = "FT")]
            pub ft: String,
            #[serde(rename = "TT")]
            pub tt: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct IoStructureOptionsDescription {
            #[serde(rename = "a")]
            pub a: String,
            #[serde(rename = "f")]
            pub f: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Signal {
            #[serde(rename = "name")]
            pub name: String,
            #[serde(rename = "instance")]
            pub instance: String,
            #[serde(rename = "die_pad")]
            pub die_pad: String,
            #[serde(rename = "function")]
            pub function: Function,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Function {
            #[serde(rename = "type")]
            pub type_field: String,
            #[serde(rename = "id")]
            pub id: Option<String>,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Bond {
            #[serde(rename = "die_pad")]
            pub die_pad: String,
            #[serde(rename = "position")]
            pub position: String,
            #[serde(rename = "sharing")]
            pub sharing: Option<Sharing>,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Sharing {
            #[serde(rename = "signals")]
            pub signals: Vec<String>,
        }
    }
}

struct PackageDirectory {
    root: PathBuf,
    pinouts: HashMap<PathBuf, Pinout>,
}

impl PackageDirectory {
    fn load_pinout(&mut self, f: &Path) -> anyhow::Result<Pinout> {
        match self.pinouts.entry(f.to_path_buf()) {
            Entry::Occupied(e) => Ok(e.get().clone()),
            Entry::Vacant(e) => {
                let parsed: Pinout = quick_xml::de::from_str(&std::fs::read_to_string(f)?)?;

                e.insert(parsed.clone());

                Ok(parsed)
            }
        }
    }

    // TODO: load other schemas
}

fn parse_psdc(
    f: PathBuf,
    d: &mut PackageDirectory,
    chips: &mut HashMap<String, Chip>,
    chip_groups: &mut Vec<ChipGroup>,
) -> anyhow::Result<()> {
    let mut groups: HashMap<String, ChipGroup> = HashMap::new();

    let parsed: Package = quick_xml::de::from_str(&std::fs::read_to_string(f)?)?;

    for family in parsed.devices.families {
        for subfamily in family.sub_families {
            for device in subfamily.devices {
                // TODO: use extra-attribute: PPN to make chipgroup
                for (variant, descriptors, extra_attributes) in device.variants.iter().map(|variant| {
                    let chain_all = || {
                        family
                            .environments
                            .iter()
                            .chain(subfamily.environments.iter())
                            .chain(device.environments.iter())
                            .chain(variant.environments.iter())
                    };

                    let descriptors: HashMap<_, _> = chain_all()
                        .map(|e| e.device.descriptors.descriptors.iter())
                        .flatten()
                        .map(|d| (&d.schema_type, d))
                        .collect();

                    let extra_attributes: HashMap<_, _> = chain_all()
                        .map(|e| e.device.extra_attributes.extra_attributes.iter())
                        .flatten()
                        .map(|a| (&a.name, a))
                        .collect();

                    (variant, descriptors, extra_attributes)
                }) {
                    let Some(ppn) = extra_attributes.get(&"PPN".to_string()) else {
                        continue;
                    };

                    groups.entry(ppn.name.clone()).or_insert_with(|| ChipGroup {
                        chip_names: Vec::new(),
                        xml: todo!(),
                        ips: HashMap::new(),
                        pins: HashMap::new(),
                        family: todo!(),
                        line: todo!(),
                        die: todo!(),
                    });

                    // for each chip variant with a unique pinout file

                    // TODO: load pinout files
                    // TODO: build `xml::chip` and `xml::chipgroup` and insert them

                    todo!()
                }
            }
        }
    }

    Ok(())
}
