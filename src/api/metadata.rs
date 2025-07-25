// TODO: This is not a comprehensive definition for the API metadata, just for PoC.
use anyhow::{anyhow, bail, Result};
use serde::Deserialize;

use crate::arg::CliInput;

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    pub plane: Plane,
    #[serde(rename = "commandGroups")]
    pub command_groups: Vec<CommandGroup>,
    pub resources: Vec<Resource>,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum Plane {
    #[serde(rename = "mgmt-plane")]
    Mgmt,
    #[serde(rename = "data-plane")]
    Data,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Method {
    Head,
    Get,
    Put,
    Patch,
    Post,
    Delete,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct CommandGroup {
    pub name: String,
    pub commands: Vec<Command>,
    // TODO: pub sub_groups: Vec<CommandGroup>,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct Command {
    pub resources: Vec<Resource>,
    pub name: String,
    pub version: String,
    #[serde(rename = "argGroups")]
    pub arg_groups: Vec<ArgGroup>,
    pub operations: Vec<Operation>,
    pub outputs: Option<Vec<Output>>,
    pub confirmation: Option<String>,
    // TODO: Need confirm if this exists
    pub command_groups: Option<Vec<CommandGroup>>,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct Resource {
    pub id: String,
    pub version: String,
    pub swagger: String,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct ArgGroup {
    pub name: String,
    pub args: Vec<Arg>,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct Arg {
    #[serde(rename = "type")]
    pub type_: String,
    pub var: String,
    pub options: Vec<String>,
    pub group: Option<String>,
    pub help: Option<Help>,
    pub required: Option<bool>,
    #[serde(rename = "idPart")]
    pub id_part: Option<String>,
    #[serde(rename = "additionalProps")]
    pub additional_props: Option<AdditionalPropSchema>,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct Help {
    pub short: String,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct Operation {
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    pub http: Option<Http>,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct Http {
    pub path: String,
    pub request: Request,
    pub responses: Vec<Response>,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct Request {
    pub method: Method,
    pub path: RequestPath,
    pub query: RequestQuery,
    pub body: Option<Body>,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct RequestPath {
    pub params: Vec<RequestPathParam>,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct RequestPathParam {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    pub arg: String,
    pub required: Option<bool>,
    pub format: Option<RequestFormat>,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct RequestFormat {
    pub pattern: String,
    #[serde(rename = "maxLength")]
    pub max_length: i64,
    #[serde(rename = "minLength")]
    pub min_length: i64,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct ResponseFormat {
    pub template: String,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct RequestQuery {
    pub consts: Vec<RequestQueryConst>,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct RequestQueryConst {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub required: Option<bool>,
    #[serde(rename = "readOnly")]
    pub read_only: Option<bool>,
    #[serde(rename = "const")]
    pub const_: bool,
    pub default: DefaultValue,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct DefaultValue {
    pub value: String,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct Body {
    pub json: BodyJSON,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct BodyJSON {
    pub schema: Option<Schema>,
    // Only applies for response body
    pub var: Option<String>,
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(rename = "statusCode")]
    pub status_code: Option<Vec<i64>>,
    pub body: Option<Body>,
    #[serde(rename = "isError")]
    pub is_error: Option<bool>,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct Output {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    #[serde(rename = "clientFlatten")]
    pub client_flatten: Option<bool>,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct Schema {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: Option<String>,
    pub required: Option<bool>,
    pub arg: Option<String>,
    #[serde(rename = "readOnly")]
    pub read_only: Option<bool>,
    pub props: Option<Vec<Schema>>,
    pub format: Option<ResponseFormat>,
    #[serde(rename = "clientFlatten")]
    pub client_flatten: Option<bool>,
    #[serde(rename = "additionalProps")]
    pub additional_props: Option<AdditionalPropSchema>,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct AdditionalPropSchema {
    pub item: AdditionalPropItemSchema,
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Clone, Deserialize)]
pub struct AdditionalPropItemSchema {
    #[serde(rename = "type")]
    pub type_: String,
}

pub trait CommandHelper {
    fn help(&self) -> String;
}

impl CommandHelper for CommandGroup {
    fn help(&self) -> String {
        format!(
            "Available commands: {:?}",
            self.commands
                .iter()
                .map(|c| c.name.clone())
                .collect::<Vec<String>>()
        )
    }
}

impl CommandHelper for &CommandGroup {
    fn help(&self) -> String {
        (*self).help()
    }
}

impl CommandHelper for Command {
    fn help(&self) -> String {
        format!(
            "Available arguments:\n\n{}",
            self.arg_groups
                .iter()
                .flat_map(|c| c.args.iter().map(|arg| format!(
                    "- {}: {}",
                    arg.options.join(","),
                    arg.type_
                )))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl CommandHelper for &Command {
    fn help(&self) -> String {
        (*self).help()
    }
}

#[derive(Debug, Clone)]
pub enum CommandOrCommandGroup {
    Command(Command),
    CommandGroup(CommandGroup),
}

impl CommandOrCommandGroup {
    pub fn as_command(&self) -> &Command {
        match self {
            CommandOrCommandGroup::Command(c) => c,
            CommandOrCommandGroup::CommandGroup(_) => panic!("this is a CommandGroup"),
        }
    }

    pub fn as_command_group(&self) -> &CommandGroup {
        match self {
            CommandOrCommandGroup::Command(_) => panic!("this is a Command"),
            CommandOrCommandGroup::CommandGroup(cg) => cg,
        }
    }
}

impl Metadata {
    pub fn resolve_command_or_command_group(
        &self,
        input: &CliInput,
    ) -> Result<CommandOrCommandGroup> {
        if input.is_empty() {
            bail!("empty CLI input");
        }

        let args = input.pos_args();
        let mut args = args.iter();

        // The first is a command group name
        let mut cg = self
            .command_groups
            .iter()
            .find(|c| c.name.as_str() == *args.next().unwrap())
            .cloned();
        let mut c: Option<Command> = None;

        let mut unknown_arg = "";
        while let Some(arg) = args.next() {
            if cg.is_some() {
                c = cg
                    .unwrap()
                    .commands
                    .iter()
                    .find(|c| c.name.as_str() == *arg)
                    .cloned();
                cg = None;
            } else if c.is_some() {
                cg = c.unwrap().command_groups.and_then(|cg| {
                    cg.iter()
                        .find(|c| c.name.as_str() == *args.next().unwrap())
                        .cloned()
                });
                c = None;
            } else {
                unknown_arg = arg;
                break;
            }
        }

        if let Some(c) = c {
            Ok(CommandOrCommandGroup::Command(c))
        } else if let Some(cg) = cg {
            Ok(CommandOrCommandGroup::CommandGroup(cg))
        } else {
            Err(anyhow!("unknown argument {}", unknown_arg))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde::Serialize;
    use serde_json::Value;
    use std::error::Error;

    // This is used to skip optional fields when they are None.
    fn to_clean_json<T: Serialize>(value: &T) -> Value {
        fn strip_nulls(value: Value) -> Value {
            match value {
                Value::Object(map) => {
                    let cleaned = map
                        .into_iter()
                        .filter_map(|(k, v)| {
                            let v = strip_nulls(v);
                            if v.is_null() {
                                None
                            } else {
                                Some((k, v))
                            }
                        })
                        .collect();
                    Value::Object(cleaned)
                }
                Value::Array(arr) => {
                    let cleaned = arr
                        .into_iter()
                        .map(strip_nulls)
                        .filter(|v| !v.is_null())
                        .collect();
                    Value::Array(cleaned)
                }
                other => other,
            }
        }

        let raw = serde_json::to_value(value).expect("Serialization failed");
        strip_nulls(raw)
    }

    #[test]
    fn deserialize() -> Result<(), Box<dyn Error>> {
        let input = r#"
{
  "plane": "mgmt-plane",
  "resources": [
    {
      "id": "/subscriptions/{}/resourcegroups/{}",
      "version": "2024-11-01",
      "swagger": "mgmt-plane/resources/ResourceProviders/Microsoft.Resources/Paths/L3N1YnNjcmlwdGlvbnMve3N1YnNjcmlwdGlvbklkfS9yZXNvdXJjZWdyb3Vwcy97cmVzb3VyY2VHcm91cE5hbWV9/V/MjAyNC0xMS0wMQ=="
    }
  ],
  "commandGroups": [
    {
      "name": "group",
      "commands": [
        {
          "name": "show",
          "version": "2024-11-01",
          "resources": [
            {
              "id": "/subscriptions/{}/resourcegroups/{}",
              "version": "2024-11-01",
              "swagger": "mgmt-plane/resources/ResourceProviders/Microsoft.Resources/Paths/L3N1YnNjcmlwdGlvbnMve3N1YnNjcmlwdGlvbklkfS9yZXNvdXJjZWdyb3Vwcy97cmVzb3VyY2VHcm91cE5hbWV9/V/MjAyNC0xMS0wMQ=="
            }
          ],
          "argGroups": [
            {
              "name": "",
              "args": [
                {
                  "type": "ResourceGroupName",
                  "var": "$Path.resourceGroupName",
                  "options": [
                    "g",
                    "resource-group"
                  ],
                  "required": true,
                  "idPart": "resource_group"
                },
                {
                  "type": "SubscriptionId",
                  "var": "$Path.subscriptionId",
                  "options": [
                    "subscription"
                  ],
                  "required": true,
                  "idPart": "subscription"
                }
              ]
            }
          ],
          "operations": [
            {
              "operationId": "ResourceGroups_Get",
              "http": {
                "path": "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}",
                "request": {
                  "method": "get",
                  "path": {
                    "params": [
                      {
                        "type": "string",
                        "name": "resourceGroupName",
                        "arg": "$Path.resourceGroupName",
                        "required": true,
                        "format": {
                          "pattern": "^[-\\w\\._\\(\\)]+$",
                          "maxLength": 90,
                          "minLength": 1
                        }
                      },
                      {
                        "type": "string",
                        "name": "subscriptionId",
                        "arg": "$Path.subscriptionId",
                        "required": true
                      }
                    ]
                  },
                  "query": {
                    "consts": [
                      {
                        "readOnly": true,
                        "const": true,
                        "default": {
                          "value": "2024-11-01"
                        },
                        "type": "string",
                        "name": "api-version",
                        "required": true
                      }
                    ]
                  }
                },
                "responses": [
                  {
                    "statusCode": [
                      200
                    ],
                    "body": {
                      "json": {
                        "var": "$Instance",
                        "schema": {
                          "type": "object",
                          "props": [
                            {
                              "readOnly": true,
                              "type": "ResourceId",
                              "name": "id",
                              "format": {
                                "template": "/subscriptions/{}/resourcegroups/{}"
                              }
                            },
                            {
                              "type": "ResourceLocation",
                              "name": "location",
                              "required": true
                            },
                            {
                              "type": "string",
                              "name": "managedBy"
                            },
                            {
                              "readOnly": true,
                              "type": "string",
                              "name": "name"
                            },
                            {
                              "type": "object",
                              "name": "properties",
                              "props": [
                                {
                                  "readOnly": true,
                                  "type": "string",
                                  "name": "provisioningState"
                                }
                              ]
                            },
                            {
                              "type": "object",
                              "name": "tags",
                              "additionalProps": {
                                "item": {
                                  "type": "string"
                                }
                              }
                            },
                            {
                              "readOnly": true,
                              "type": "string",
                              "name": "type"
                            }
                          ]
                        }
                      }
                    }
                  },
                  {
                    "isError": true,
                    "body": {
                      "json": {
                        "schema": {
                          "type": "@MgmtErrorFormat"
                        }
                      }
                    }
                  }
                ]
              }
            }
          ],
          "outputs": [
            {
              "type": "object",
              "ref": "$Instance",
              "clientFlatten": true
            }
          ]
        },
        {
          "name": "create",
          "version": "2024-11-01",
          "resources": [
            {
              "id": "/subscriptions/{}/resourcegroups/{}",
              "version": "2024-11-01",
              "swagger": "mgmt-plane/resources/ResourceProviders/Microsoft.Resources/Paths/L3N1YnNjcmlwdGlvbnMve3N1YnNjcmlwdGlvbklkfS9yZXNvdXJjZWdyb3Vwcy97cmVzb3VyY2VHcm91cE5hbWV9/V/MjAyNC0xMS0wMQ=="
            }
          ],
          "argGroups": [
            {
              "name": "",
              "args": [
                {
                  "type": "ResourceGroupName",
                  "var": "$Path.resourceGroupName",
                  "options": [
                    "g",
                    "resource-group"
                  ],
                  "required": true,
                  "idPart": "resource_group"
                },
                {
                  "type": "SubscriptionId",
                  "var": "$Path.subscriptionId",
                  "options": [
                    "subscription"
                  ],
                  "required": true,
                  "idPart": "subscription"
                }
              ]
            },
            {
              "name": "Parameters",
              "args": [
                {
                  "type": "ResourceLocation",
                  "var": "$parameters.location",
                  "options": [
                    "l",
                    "location"
                  ],
                  "required": true,
                  "group": "Parameters",
                  "help": {
                    "short": "The location of the resource group. It cannot be changed after the resource group has been created. It must be one of the supported Azure locations."
                  }
                },
                {
                  "type": "string",
                  "var": "$parameters.managedBy",
                  "options": [
                    "managed-by"
                  ],
                  "group": "Parameters",
                  "help": {
                    "short": "The ID of the resource that manages this resource group."
                  }
                },
                {
                  "type": "object",
                  "var": "$parameters.tags",
                  "options": [
                    "tags"
                  ],
                  "group": "Parameters",
                  "help": {
                    "short": "The tags attached to the resource group."
                  },
                  "additionalProps": {
                    "item": {
                      "type": "string"
                    }
                  }
                }
              ]
            }
          ],
          "operations": [
            {
              "operationId": "ResourceGroups_CreateOrUpdate",
              "http": {
                "path": "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}",
                "request": {
                  "method": "put",
                  "path": {
                    "params": [
                      {
                        "type": "string",
                        "name": "resourceGroupName",
                        "arg": "$Path.resourceGroupName",
                        "required": true,
                        "format": {
                          "pattern": "^[-\\w\\._\\(\\)]+$",
                          "maxLength": 90,
                          "minLength": 1
                        }
                      },
                      {
                        "type": "string",
                        "name": "subscriptionId",
                        "arg": "$Path.subscriptionId",
                        "required": true
                      }
                    ]
                  },
                  "query": {
                    "consts": [
                      {
                        "readOnly": true,
                        "const": true,
                        "default": {
                          "value": "2024-11-01"
                        },
                        "type": "string",
                        "name": "api-version",
                        "required": true
                      }
                    ]
                  },
                  "body": {
                    "json": {
                      "schema": {
                        "type": "object",
                        "name": "parameters",
                        "required": true,
                        "props": [
                          {
                            "type": "ResourceLocation",
                            "name": "location",
                            "arg": "$parameters.location",
                            "required": true
                          },
                          {
                            "type": "string",
                            "name": "managedBy",
                            "arg": "$parameters.managedBy"
                          },
                          {
                            "type": "object",
                            "name": "tags",
                            "arg": "$parameters.tags",
                            "additionalProps": {
                              "item": {
                                "type": "string"
                              }
                            }
                          }
                        ],
                        "clientFlatten": true
                      }
                    }
                  }
                },
                "responses": [
                  {
                    "statusCode": [
                      200,
                      201
                    ],
                    "body": {
                      "json": {
                        "var": "$Instance",
                        "schema": {
                          "type": "object",
                          "props": [
                            {
                              "readOnly": true,
                              "type": "ResourceId",
                              "name": "id",
                              "format": {
                                "template": "/subscriptions/{}/resourcegroups/{}"
                              }
                            },
                            {
                              "type": "ResourceLocation",
                              "name": "location",
                              "required": true
                            },
                            {
                              "type": "string",
                              "name": "managedBy"
                            },
                            {
                              "readOnly": true,
                              "type": "string",
                              "name": "name"
                            },
                            {
                              "type": "object",
                              "name": "properties",
                              "props": [
                                {
                                  "readOnly": true,
                                  "type": "string",
                                  "name": "provisioningState"
                                }
                              ]
                            },
                            {
                              "type": "object",
                              "name": "tags",
                              "additionalProps": {
                                "item": {
                                  "type": "string"
                                }
                              }
                            },
                            {
                              "readOnly": true,
                              "type": "string",
                              "name": "type"
                            }
                          ]
                        }
                      }
                    }
                  },
                  {
                    "isError": true,
                    "body": {
                      "json": {
                        "schema": {
                          "type": "@MgmtErrorFormat"
                        }
                      }
                    }
                  }
                ]
              }
            }
          ],
          "outputs": [
            {
              "type": "object",
              "ref": "$Instance",
              "clientFlatten": true
            }
          ],
          "confirmation": ""
        }
      ]
    }
  ]
}
            "#;
        let metadata: Metadata = serde_json::from_str(input)?;
        let input_json: Value = serde_json::from_str(input)?;
        let output_json: Value = to_clean_json(&metadata);
        assert_eq!(input_json, output_json);
        Ok(())
    }
}
