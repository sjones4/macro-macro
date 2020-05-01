# macro!macro!

Rust macro templating library for cleaner code.

## Usage

Declare a named template with the macros you want to use. The template treats some items specially:

* `__` : one double underscore can be used alone as a name wildcard or for prefix and/or suffix matching
* `!` : for a type wildcard

## Examples

Name prefix matching on struct and field name:

    macro_template!(prefix_match = {
      #...
      struct STRUCT_PREFIX__ {
        #... FIELD_PREFIX__: !,
      }
    });

Type matching for field macros:

    macro_template!(field_type_match = {
      struct __ {
        #... __: i32,
      }
    });

which would add the declared attribute macro(s) to any fields in a struct with type `i32`. 

Full example for serde annotation macros:

    use macro_macro::*;
    use serde::{Deserialize, Serialize};
    
    macro_template!(serde_service_model = {
      #[derive(Default, Debug, Clone, Serialize, Deserialize)]
      #[serde(rename_all = "camelCase")]
      struct __ {
        #[serde(skip_serializing_if = "Option::is_none")] __: Option<!>,
      }
    });
    
    #[macro_macro(serde_service_model)]
    pub struct Service {
        pub version: String,
        pub metadata: Option<Metadata>,
        ...
    }
    
    #[macro_macro(serde_service_model)]
    pub struct Metadata {
        pub api_version: Option<String>,
        pub endpoint_prefix: Option<String>,
        ...
    }

which is equivalent to annotating each struct directly with:

    #[derive(Default, Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]

and each field of type `Option` with:

    #[serde(skip_serializing_if = "Option::is_none")]

## License

Licensed under:

 * MIT license
   ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any
additional terms or conditions.
