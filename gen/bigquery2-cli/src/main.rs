// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![feature(plugin, exit_status)]
#![plugin(docopt_macros)]
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

extern crate docopt;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate rustc_serialize;
extern crate serde;
extern crate hyper;
extern crate mime;
extern crate google_bigquery2 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  bigquery2 [options] datasets delete <project-id> <dataset-id> [-p <v>]...
  bigquery2 [options] datasets get <project-id> <dataset-id> [-p <v>]... [-o <out>]
  bigquery2 [options] datasets insert <project-id> -r <kv>... [-p <v>]... [-o <out>]
  bigquery2 [options] datasets list <project-id> [-p <v>]... [-o <out>]
  bigquery2 [options] datasets patch <project-id> <dataset-id> -r <kv>... [-p <v>]... [-o <out>]
  bigquery2 [options] datasets update <project-id> <dataset-id> -r <kv>... [-p <v>]... [-o <out>]
  bigquery2 [options] jobs get <project-id> <job-id> [-p <v>]... [-o <out>]
  bigquery2 [options] jobs get-query-results <project-id> <job-id> [-p <v>]... [-o <out>]
  bigquery2 [options] jobs insert <project-id> -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  bigquery2 [options] jobs list <project-id> [-p <v>]... [-o <out>]
  bigquery2 [options] jobs query <project-id> -r <kv>... [-p <v>]... [-o <out>]
  bigquery2 [options] projects list [-p <v>]... [-o <out>]
  bigquery2 [options] tabledata insert-all <project-id> <dataset-id> <table-id> -r <kv>... [-p <v>]... [-o <out>]
  bigquery2 [options] tabledata list <project-id> <dataset-id> <table-id> [-p <v>]... [-o <out>]
  bigquery2 [options] tables delete <project-id> <dataset-id> <table-id> [-p <v>]...
  bigquery2 [options] tables get <project-id> <dataset-id> <table-id> [-p <v>]... [-o <out>]
  bigquery2 [options] tables insert <project-id> <dataset-id> -r <kv>... [-p <v>]... [-o <out>]
  bigquery2 [options] tables list <project-id> <dataset-id> [-p <v>]... [-o <out>]
  bigquery2 [options] tables patch <project-id> <dataset-id> <table-id> -r <kv>... [-p <v>]... [-o <out>]
  bigquery2 [options] tables update <project-id> <dataset-id> <table-id> -r <kv>... [-p <v>]... [-o <out>]
  bigquery2 --help

All documentation details can be found TODO: <URL to github.io docs here, see #51>

Configuration:
  --scope <url>  
            Specify the authentication a method should be executed in. Each scope requires
            the user to grant this application permission to use it.
            If unset, it defaults to the shortest scope url for a particular method.
  --config-dir <folder>
            A directory into which we will store our persistent data. Defaults to a user-writable
            directory that we will create during the first invocation.
            [default: ~/.google-service-cli]
  --debug
            Output all server communication to standard error. `tx` and `rx` are placed into 
            the same stream.
  --debug-auth
            Output all communication related to authentication to standard error. `tx` and `rx` are placed into 
            the same stream.
");

mod cmn;
use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg, 
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate};
use rustc_serialize::json;

struct Engine {
    opt: Options,
    hub: api::Bigquery<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _datasets_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.datasets().delete(&self.opt.arg_project_id, &self.opt.arg_dataset_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "delete-contents" => {
                    call = call.delete_contents(arg_from_str(value.unwrap_or("false"), err, "delete-contents", "boolean"));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    None
                }
            }
        }
    }

    fn _datasets_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.datasets().get(&self.opt.arg_project_id, &self.opt.arg_dataset_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _datasets_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Dataset::default();
        let mut call = self.hub.datasets().insert(&request, &self.opt.arg_project_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_dataset_reference_init(request: &mut api::Dataset) {
                if request.dataset_reference.is_none() {
                    request.dataset_reference = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "dataset-reference.project-id" => {
                        request_dataset_reference_init(&mut request);
                        request.dataset_reference.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "dataset-reference.dataset-id" => {
                        request_dataset_reference_init(&mut request);
                        request.dataset_reference.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request_dataset_reference_init(&mut request);
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "default-table-expiration-ms" => {
                        request_dataset_reference_init(&mut request);
                        request.default_table_expiration_ms = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_dataset_reference_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "friendly-name" => {
                        request_dataset_reference_init(&mut request);
                        request.friendly_name = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request_dataset_reference_init(&mut request);
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_dataset_reference_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_dataset_reference_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _datasets_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.datasets().list(&self.opt.arg_project_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "all" => {
                    call = call.all(arg_from_str(value.unwrap_or("false"), err, "all", "boolean"));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _datasets_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Dataset::default();
        let mut call = self.hub.datasets().patch(&request, &self.opt.arg_project_id, &self.opt.arg_dataset_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_dataset_reference_init(request: &mut api::Dataset) {
                if request.dataset_reference.is_none() {
                    request.dataset_reference = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "dataset-reference.project-id" => {
                        request_dataset_reference_init(&mut request);
                        request.dataset_reference.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "dataset-reference.dataset-id" => {
                        request_dataset_reference_init(&mut request);
                        request.dataset_reference.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request_dataset_reference_init(&mut request);
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "default-table-expiration-ms" => {
                        request_dataset_reference_init(&mut request);
                        request.default_table_expiration_ms = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_dataset_reference_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "friendly-name" => {
                        request_dataset_reference_init(&mut request);
                        request.friendly_name = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request_dataset_reference_init(&mut request);
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_dataset_reference_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_dataset_reference_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _datasets_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Dataset::default();
        let mut call = self.hub.datasets().update(&request, &self.opt.arg_project_id, &self.opt.arg_dataset_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_dataset_reference_init(request: &mut api::Dataset) {
                if request.dataset_reference.is_none() {
                    request.dataset_reference = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "dataset-reference.project-id" => {
                        request_dataset_reference_init(&mut request);
                        request.dataset_reference.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "dataset-reference.dataset-id" => {
                        request_dataset_reference_init(&mut request);
                        request.dataset_reference.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request_dataset_reference_init(&mut request);
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "default-table-expiration-ms" => {
                        request_dataset_reference_init(&mut request);
                        request.default_table_expiration_ms = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_dataset_reference_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "friendly-name" => {
                        request_dataset_reference_init(&mut request);
                        request.friendly_name = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request_dataset_reference_init(&mut request);
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_dataset_reference_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_dataset_reference_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _jobs_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.jobs().get(&self.opt.arg_project_id, &self.opt.arg_job_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _jobs_get_query_results(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.jobs().get_query_results(&self.opt.arg_project_id, &self.opt.arg_job_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "timeout-ms" => {
                    call = call.timeout_ms(arg_from_str(value.unwrap_or("-0"), err, "timeout-ms", "integer"));
                },
                "start-index" => {
                    call = call.start_index(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _jobs_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Job::default();
        let mut call = self.hub.jobs().insert(&request, &self.opt.arg_project_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_configuration_copy_destination_table_init(request: &mut api::Job) {
                request_configuration_copy_init(request);
                if request.configuration.as_mut().unwrap().copy.as_mut().unwrap().destination_table.is_none() {
                    request.configuration.as_mut().unwrap().copy.as_mut().unwrap().destination_table = Some(Default::default());
                }
            }
            
            fn request_configuration_copy_init(request: &mut api::Job) {
                request_configuration_init(request);
                if request.configuration.as_mut().unwrap().copy.is_none() {
                    request.configuration.as_mut().unwrap().copy = Some(Default::default());
                }
            }
            
            fn request_configuration_copy_source_table_init(request: &mut api::Job) {
                request_configuration_copy_init(request);
                if request.configuration.as_mut().unwrap().copy.as_mut().unwrap().source_table.is_none() {
                    request.configuration.as_mut().unwrap().copy.as_mut().unwrap().source_table = Some(Default::default());
                }
            }
            
            fn request_configuration_extract_init(request: &mut api::Job) {
                request_configuration_init(request);
                if request.configuration.as_mut().unwrap().extract.is_none() {
                    request.configuration.as_mut().unwrap().extract = Some(Default::default());
                }
            }
            
            fn request_configuration_extract_source_table_init(request: &mut api::Job) {
                request_configuration_extract_init(request);
                if request.configuration.as_mut().unwrap().extract.as_mut().unwrap().source_table.is_none() {
                    request.configuration.as_mut().unwrap().extract.as_mut().unwrap().source_table = Some(Default::default());
                }
            }
            
            fn request_configuration_init(request: &mut api::Job) {
                if request.configuration.is_none() {
                    request.configuration = Some(Default::default());
                }
            }
            
            fn request_configuration_link_destination_table_init(request: &mut api::Job) {
                request_configuration_link_init(request);
                if request.configuration.as_mut().unwrap().link.as_mut().unwrap().destination_table.is_none() {
                    request.configuration.as_mut().unwrap().link.as_mut().unwrap().destination_table = Some(Default::default());
                }
            }
            
            fn request_configuration_link_init(request: &mut api::Job) {
                request_configuration_init(request);
                if request.configuration.as_mut().unwrap().link.is_none() {
                    request.configuration.as_mut().unwrap().link = Some(Default::default());
                }
            }
            
            fn request_configuration_load_destination_table_init(request: &mut api::Job) {
                request_configuration_load_init(request);
                if request.configuration.as_mut().unwrap().load.as_mut().unwrap().destination_table.is_none() {
                    request.configuration.as_mut().unwrap().load.as_mut().unwrap().destination_table = Some(Default::default());
                }
            }
            
            fn request_configuration_load_init(request: &mut api::Job) {
                request_configuration_init(request);
                if request.configuration.as_mut().unwrap().load.is_none() {
                    request.configuration.as_mut().unwrap().load = Some(Default::default());
                }
            }
            
            fn request_configuration_query_default_dataset_init(request: &mut api::Job) {
                request_configuration_query_init(request);
                if request.configuration.as_mut().unwrap().query.as_mut().unwrap().default_dataset.is_none() {
                    request.configuration.as_mut().unwrap().query.as_mut().unwrap().default_dataset = Some(Default::default());
                }
            }
            
            fn request_configuration_query_destination_table_init(request: &mut api::Job) {
                request_configuration_query_init(request);
                if request.configuration.as_mut().unwrap().query.as_mut().unwrap().destination_table.is_none() {
                    request.configuration.as_mut().unwrap().query.as_mut().unwrap().destination_table = Some(Default::default());
                }
            }
            
            fn request_configuration_query_init(request: &mut api::Job) {
                request_configuration_init(request);
                if request.configuration.as_mut().unwrap().query.is_none() {
                    request.configuration.as_mut().unwrap().query = Some(Default::default());
                }
            }
            
            fn request_job_reference_init(request: &mut api::Job) {
                if request.job_reference.is_none() {
                    request.job_reference = Some(Default::default());
                }
            }
            
            fn request_statistics_extract_init(request: &mut api::Job) {
                request_statistics_init(request);
                if request.statistics.as_mut().unwrap().extract.is_none() {
                    request.statistics.as_mut().unwrap().extract = Some(Default::default());
                }
            }
            
            fn request_statistics_init(request: &mut api::Job) {
                if request.statistics.is_none() {
                    request.statistics = Some(Default::default());
                }
            }
            
            fn request_statistics_load_init(request: &mut api::Job) {
                request_statistics_init(request);
                if request.statistics.as_mut().unwrap().load.is_none() {
                    request.statistics.as_mut().unwrap().load = Some(Default::default());
                }
            }
            
            fn request_statistics_query_init(request: &mut api::Job) {
                request_statistics_init(request);
                if request.statistics.as_mut().unwrap().query.is_none() {
                    request.statistics.as_mut().unwrap().query = Some(Default::default());
                }
            }
            
            fn request_status_error_result_init(request: &mut api::Job) {
                request_status_init(request);
                if request.status.as_mut().unwrap().error_result.is_none() {
                    request.status.as_mut().unwrap().error_result = Some(Default::default());
                }
            }
            
            fn request_status_init(request: &mut api::Job) {
                if request.status.is_none() {
                    request.status = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "status.state" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().state = Some(value.unwrap_or("").to_string());
                    },
                "status.error-result.debug-info" => {
                        request_status_error_result_init(&mut request);
                        request.status.as_mut().unwrap().error_result.as_mut().unwrap().debug_info = Some(value.unwrap_or("").to_string());
                    },
                "status.error-result.message" => {
                        request_status_error_result_init(&mut request);
                        request.status.as_mut().unwrap().error_result.as_mut().unwrap().message = Some(value.unwrap_or("").to_string());
                    },
                "status.error-result.reason" => {
                        request_status_error_result_init(&mut request);
                        request.status.as_mut().unwrap().error_result.as_mut().unwrap().reason = Some(value.unwrap_or("").to_string());
                    },
                "status.error-result.location" => {
                        request_status_error_result_init(&mut request);
                        request.status.as_mut().unwrap().error_result.as_mut().unwrap().location = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_status_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "statistics.load.output-rows" => {
                        request_statistics_load_init(&mut request);
                        request.statistics.as_mut().unwrap().load.as_mut().unwrap().output_rows = Some(value.unwrap_or("").to_string());
                    },
                "statistics.load.input-files" => {
                        request_statistics_load_init(&mut request);
                        request.statistics.as_mut().unwrap().load.as_mut().unwrap().input_files = Some(value.unwrap_or("").to_string());
                    },
                "statistics.load.input-file-bytes" => {
                        request_statistics_load_init(&mut request);
                        request.statistics.as_mut().unwrap().load.as_mut().unwrap().input_file_bytes = Some(value.unwrap_or("").to_string());
                    },
                "statistics.load.output-bytes" => {
                        request_statistics_load_init(&mut request);
                        request.statistics.as_mut().unwrap().load.as_mut().unwrap().output_bytes = Some(value.unwrap_or("").to_string());
                    },
                "statistics.creation-time" => {
                        request_statistics_load_init(&mut request);
                        request.statistics.as_mut().unwrap().creation_time = Some(value.unwrap_or("").to_string());
                    },
                "statistics.total-bytes-processed" => {
                        request_statistics_load_init(&mut request);
                        request.statistics.as_mut().unwrap().total_bytes_processed = Some(value.unwrap_or("").to_string());
                    },
                "statistics.start-time" => {
                        request_statistics_load_init(&mut request);
                        request.statistics.as_mut().unwrap().start_time = Some(value.unwrap_or("").to_string());
                    },
                "statistics.query.cache-hit" => {
                        request_statistics_query_init(&mut request);
                        request.statistics.as_mut().unwrap().query.as_mut().unwrap().cache_hit = Some(arg_from_str(value.unwrap_or("false"), err, "statistics.query.cache-hit", "boolean"));
                    },
                "statistics.query.total-bytes-processed" => {
                        request_statistics_query_init(&mut request);
                        request.statistics.as_mut().unwrap().query.as_mut().unwrap().total_bytes_processed = Some(value.unwrap_or("").to_string());
                    },
                "statistics.end-time" => {
                        request_statistics_query_init(&mut request);
                        request.statistics.as_mut().unwrap().end_time = Some(value.unwrap_or("").to_string());
                    },
                "statistics.extract.destination-uri-file-counts" => {
                        request_statistics_extract_init(&mut request);
                        if request.statistics.as_mut().unwrap().extract.as_mut().unwrap().destination_uri_file_counts.is_none() {
                           request.statistics.as_mut().unwrap().extract.as_mut().unwrap().destination_uri_file_counts = Some(Default::default());
                        }
                                        request.statistics.as_mut().unwrap().extract.as_mut().unwrap().destination_uri_file_counts.as_mut().unwrap().push(arg_from_str(value.unwrap_or("-0"), err, "statistics.extract.destination-uri-file-counts", "int64"));
                    },
                "job-reference.project-id" => {
                        request_job_reference_init(&mut request);
                        request.job_reference.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "job-reference.job-id" => {
                        request_job_reference_init(&mut request);
                        request.job_reference.as_mut().unwrap().job_id = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_job_reference_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "user-email" => {
                        request_job_reference_init(&mut request);
                        request.user_email = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.encoding" => {
                        request_configuration_load_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().encoding = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.skip-leading-rows" => {
                        request_configuration_load_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().skip_leading_rows = Some(arg_from_str(value.unwrap_or("-0"), err, "configuration.load.skip-leading-rows", "integer"));
                    },
                "configuration.load.quote" => {
                        request_configuration_load_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().quote = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.source-format" => {
                        request_configuration_load_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().source_format = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.destination-table.project-id" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().destination_table.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.destination-table.table-id" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().destination_table.as_mut().unwrap().table_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.destination-table.dataset-id" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().destination_table.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.max-bad-records" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().max_bad_records = Some(arg_from_str(value.unwrap_or("-0"), err, "configuration.load.max-bad-records", "integer"));
                    },
                "configuration.load.allow-jagged-rows" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().allow_jagged_rows = Some(arg_from_str(value.unwrap_or("false"), err, "configuration.load.allow-jagged-rows", "boolean"));
                    },
                "configuration.load.write-disposition" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().write_disposition = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.source-uris" => {
                        request_configuration_load_destination_table_init(&mut request);
                        if request.configuration.as_mut().unwrap().load.as_mut().unwrap().source_uris.is_none() {
                           request.configuration.as_mut().unwrap().load.as_mut().unwrap().source_uris = Some(Default::default());
                        }
                                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().source_uris.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "configuration.load.field-delimiter" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().field_delimiter = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.create-disposition" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().create_disposition = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.schema-inline-format" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().schema_inline_format = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.schema-inline" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().schema_inline = Some(value.unwrap_or("").to_string());
                    },
                "configuration.load.allow-quoted-newlines" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().allow_quoted_newlines = Some(arg_from_str(value.unwrap_or("false"), err, "configuration.load.allow-quoted-newlines", "boolean"));
                    },
                "configuration.load.projection-fields" => {
                        request_configuration_load_destination_table_init(&mut request);
                        if request.configuration.as_mut().unwrap().load.as_mut().unwrap().projection_fields.is_none() {
                           request.configuration.as_mut().unwrap().load.as_mut().unwrap().projection_fields = Some(Default::default());
                        }
                                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().projection_fields.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "configuration.load.ignore-unknown-values" => {
                        request_configuration_load_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().load.as_mut().unwrap().ignore_unknown_values = Some(arg_from_str(value.unwrap_or("false"), err, "configuration.load.ignore-unknown-values", "boolean"));
                    },
                "configuration.dry-run" => {
                        request_configuration_load_init(&mut request);
                        request.configuration.as_mut().unwrap().dry_run = Some(arg_from_str(value.unwrap_or("false"), err, "configuration.dry-run", "boolean"));
                    },
                "configuration.link.create-disposition" => {
                        request_configuration_link_init(&mut request);
                        request.configuration.as_mut().unwrap().link.as_mut().unwrap().create_disposition = Some(value.unwrap_or("").to_string());
                    },
                "configuration.link.write-disposition" => {
                        request_configuration_link_init(&mut request);
                        request.configuration.as_mut().unwrap().link.as_mut().unwrap().write_disposition = Some(value.unwrap_or("").to_string());
                    },
                "configuration.link.destination-table.project-id" => {
                        request_configuration_link_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().link.as_mut().unwrap().destination_table.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.link.destination-table.table-id" => {
                        request_configuration_link_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().link.as_mut().unwrap().destination_table.as_mut().unwrap().table_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.link.destination-table.dataset-id" => {
                        request_configuration_link_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().link.as_mut().unwrap().destination_table.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.link.source-uri" => {
                        request_configuration_link_destination_table_init(&mut request);
                        if request.configuration.as_mut().unwrap().link.as_mut().unwrap().source_uri.is_none() {
                           request.configuration.as_mut().unwrap().link.as_mut().unwrap().source_uri = Some(Default::default());
                        }
                                        request.configuration.as_mut().unwrap().link.as_mut().unwrap().source_uri.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "configuration.query.flatten-results" => {
                        request_configuration_query_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().flatten_results = Some(arg_from_str(value.unwrap_or("false"), err, "configuration.query.flatten-results", "boolean"));
                    },
                "configuration.query.use-query-cache" => {
                        request_configuration_query_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().use_query_cache = Some(arg_from_str(value.unwrap_or("false"), err, "configuration.query.use-query-cache", "boolean"));
                    },
                "configuration.query.default-dataset.project-id" => {
                        request_configuration_query_default_dataset_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().default_dataset.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.query.default-dataset.dataset-id" => {
                        request_configuration_query_default_dataset_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().default_dataset.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.query.destination-table.project-id" => {
                        request_configuration_query_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().destination_table.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.query.destination-table.table-id" => {
                        request_configuration_query_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().destination_table.as_mut().unwrap().table_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.query.destination-table.dataset-id" => {
                        request_configuration_query_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().destination_table.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.query.priority" => {
                        request_configuration_query_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().priority = Some(value.unwrap_or("").to_string());
                    },
                "configuration.query.write-disposition" => {
                        request_configuration_query_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().write_disposition = Some(value.unwrap_or("").to_string());
                    },
                "configuration.query.allow-large-results" => {
                        request_configuration_query_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().allow_large_results = Some(arg_from_str(value.unwrap_or("false"), err, "configuration.query.allow-large-results", "boolean"));
                    },
                "configuration.query.create-disposition" => {
                        request_configuration_query_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().create_disposition = Some(value.unwrap_or("").to_string());
                    },
                "configuration.query.query" => {
                        request_configuration_query_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().query = Some(value.unwrap_or("").to_string());
                    },
                "configuration.query.preserve-nulls" => {
                        request_configuration_query_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().query.as_mut().unwrap().preserve_nulls = Some(arg_from_str(value.unwrap_or("false"), err, "configuration.query.preserve-nulls", "boolean"));
                    },
                "configuration.copy.create-disposition" => {
                        request_configuration_copy_init(&mut request);
                        request.configuration.as_mut().unwrap().copy.as_mut().unwrap().create_disposition = Some(value.unwrap_or("").to_string());
                    },
                "configuration.copy.write-disposition" => {
                        request_configuration_copy_init(&mut request);
                        request.configuration.as_mut().unwrap().copy.as_mut().unwrap().write_disposition = Some(value.unwrap_or("").to_string());
                    },
                "configuration.copy.destination-table.project-id" => {
                        request_configuration_copy_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().copy.as_mut().unwrap().destination_table.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.copy.destination-table.table-id" => {
                        request_configuration_copy_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().copy.as_mut().unwrap().destination_table.as_mut().unwrap().table_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.copy.destination-table.dataset-id" => {
                        request_configuration_copy_destination_table_init(&mut request);
                        request.configuration.as_mut().unwrap().copy.as_mut().unwrap().destination_table.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.copy.source-table.project-id" => {
                        request_configuration_copy_source_table_init(&mut request);
                        request.configuration.as_mut().unwrap().copy.as_mut().unwrap().source_table.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.copy.source-table.table-id" => {
                        request_configuration_copy_source_table_init(&mut request);
                        request.configuration.as_mut().unwrap().copy.as_mut().unwrap().source_table.as_mut().unwrap().table_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.copy.source-table.dataset-id" => {
                        request_configuration_copy_source_table_init(&mut request);
                        request.configuration.as_mut().unwrap().copy.as_mut().unwrap().source_table.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.extract.destination-uri" => {
                        request_configuration_extract_init(&mut request);
                        request.configuration.as_mut().unwrap().extract.as_mut().unwrap().destination_uri = Some(value.unwrap_or("").to_string());
                    },
                "configuration.extract.compression" => {
                        request_configuration_extract_init(&mut request);
                        request.configuration.as_mut().unwrap().extract.as_mut().unwrap().compression = Some(value.unwrap_or("").to_string());
                    },
                "configuration.extract.field-delimiter" => {
                        request_configuration_extract_init(&mut request);
                        request.configuration.as_mut().unwrap().extract.as_mut().unwrap().field_delimiter = Some(value.unwrap_or("").to_string());
                    },
                "configuration.extract.destination-format" => {
                        request_configuration_extract_init(&mut request);
                        request.configuration.as_mut().unwrap().extract.as_mut().unwrap().destination_format = Some(value.unwrap_or("").to_string());
                    },
                "configuration.extract.print-header" => {
                        request_configuration_extract_init(&mut request);
                        request.configuration.as_mut().unwrap().extract.as_mut().unwrap().print_header = Some(arg_from_str(value.unwrap_or("false"), err, "configuration.extract.print-header", "boolean"));
                    },
                "configuration.extract.destination-uris" => {
                        request_configuration_extract_init(&mut request);
                        if request.configuration.as_mut().unwrap().extract.as_mut().unwrap().destination_uris.is_none() {
                           request.configuration.as_mut().unwrap().extract.as_mut().unwrap().destination_uris = Some(Default::default());
                        }
                                        request.configuration.as_mut().unwrap().extract.as_mut().unwrap().destination_uris.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "configuration.extract.source-table.project-id" => {
                        request_configuration_extract_source_table_init(&mut request);
                        request.configuration.as_mut().unwrap().extract.as_mut().unwrap().source_table.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.extract.source-table.table-id" => {
                        request_configuration_extract_source_table_init(&mut request);
                        request.configuration.as_mut().unwrap().extract.as_mut().unwrap().source_table.as_mut().unwrap().table_id = Some(value.unwrap_or("").to_string());
                    },
                "configuration.extract.source-table.dataset-id" => {
                        request_configuration_extract_source_table_init(&mut request);
                        request.configuration.as_mut().unwrap().extract.as_mut().unwrap().source_table.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_configuration_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_configuration_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = 
            if self.opt.cmd_simple {
                "simple"
            } else if self.opt.cmd_resumable {
                "resumable"
            } else { 
                unreachable!() 
            };
        let mut input_file = input_file_from_opts(&self.opt.arg_file, err);
        let mime_type = input_mime_from_opts(&self.opt.arg_mime, err);
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "simple" => call.upload(input_file.unwrap(), mime_type.unwrap()),
                "resumable" => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _jobs_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.jobs().list(&self.opt.arg_project_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "state-filter" => {
                    call = call.add_state_filter(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "all-users" => {
                    call = call.all_users(arg_from_str(value.unwrap_or("false"), err, "all-users", "boolean"));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _jobs_query(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::QueryRequest::default();
        let mut call = self.hub.jobs().query(&request, &self.opt.arg_project_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_default_dataset_init(request: &mut api::QueryRequest) {
                if request.default_dataset.is_none() {
                    request.default_dataset = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "timeout-ms" => {
                        request.timeout_ms = Some(arg_from_str(value.unwrap_or("-0"), err, "timeout-ms", "integer"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "dry-run" => {
                        request.dry_run = Some(arg_from_str(value.unwrap_or("false"), err, "dry-run", "boolean"));
                    },
                "use-query-cache" => {
                        request.use_query_cache = Some(arg_from_str(value.unwrap_or("false"), err, "use-query-cache", "boolean"));
                    },
                "default-dataset.project-id" => {
                        request_default_dataset_init(&mut request);
                        request.default_dataset.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "default-dataset.dataset-id" => {
                        request_default_dataset_init(&mut request);
                        request.default_dataset.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "max-results" => {
                        request_default_dataset_init(&mut request);
                        request.max_results = Some(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                    },
                "query" => {
                        request_default_dataset_init(&mut request);
                        request.query = Some(value.unwrap_or("").to_string());
                    },
                "preserve-nulls" => {
                        request_default_dataset_init(&mut request);
                        request.preserve_nulls = Some(arg_from_str(value.unwrap_or("false"), err, "preserve-nulls", "boolean"));
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _projects_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _tabledata_insert_all(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::TableDataInsertAllRequest::default();
        let mut call = self.hub.tabledata().insert_all(&request, &self.opt.arg_project_id, &self.opt.arg_dataset_id, &self.opt.arg_table_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
                "ignore-unknown-values" => {
                        request.ignore_unknown_values = Some(arg_from_str(value.unwrap_or("false"), err, "ignore-unknown-values", "boolean"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "skip-invalid-rows" => {
                        request.skip_invalid_rows = Some(arg_from_str(value.unwrap_or("false"), err, "skip-invalid-rows", "boolean"));
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _tabledata_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.tabledata().list(&self.opt.arg_project_id, &self.opt.arg_dataset_id, &self.opt.arg_table_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _tables_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.tables().delete(&self.opt.arg_project_id, &self.opt.arg_dataset_id, &self.opt.arg_table_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    None
                }
            }
        }
    }

    fn _tables_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.tables().get(&self.opt.arg_project_id, &self.opt.arg_dataset_id, &self.opt.arg_table_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _tables_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Table::default();
        let mut call = self.hub.tables().insert(&request, &self.opt.arg_project_id, &self.opt.arg_dataset_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_table_reference_init(request: &mut api::Table) {
                if request.table_reference.is_none() {
                    request.table_reference = Some(Default::default());
                }
            }
            
            fn request_view_init(request: &mut api::Table) {
                if request.view.is_none() {
                    request.view = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "table-reference.project-id" => {
                        request_table_reference_init(&mut request);
                        request.table_reference.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "table-reference.table-id" => {
                        request_table_reference_init(&mut request);
                        request.table_reference.as_mut().unwrap().table_id = Some(value.unwrap_or("").to_string());
                    },
                "table-reference.dataset-id" => {
                        request_table_reference_init(&mut request);
                        request.table_reference.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "num-rows" => {
                        request_table_reference_init(&mut request);
                        request.num_rows = Some(value.unwrap_or("").to_string());
                    },
                "num-bytes" => {
                        request_table_reference_init(&mut request);
                        request.num_bytes = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_table_reference_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "friendly-name" => {
                        request_table_reference_init(&mut request);
                        request.friendly_name = Some(value.unwrap_or("").to_string());
                    },
                "expiration-time" => {
                        request_table_reference_init(&mut request);
                        request.expiration_time = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_table_reference_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_table_reference_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_table_reference_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "view.query" => {
                        request_view_init(&mut request);
                        request.view.as_mut().unwrap().query = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _tables_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.tables().list(&self.opt.arg_project_id, &self.opt.arg_dataset_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _tables_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Table::default();
        let mut call = self.hub.tables().patch(&request, &self.opt.arg_project_id, &self.opt.arg_dataset_id, &self.opt.arg_table_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_table_reference_init(request: &mut api::Table) {
                if request.table_reference.is_none() {
                    request.table_reference = Some(Default::default());
                }
            }
            
            fn request_view_init(request: &mut api::Table) {
                if request.view.is_none() {
                    request.view = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "table-reference.project-id" => {
                        request_table_reference_init(&mut request);
                        request.table_reference.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "table-reference.table-id" => {
                        request_table_reference_init(&mut request);
                        request.table_reference.as_mut().unwrap().table_id = Some(value.unwrap_or("").to_string());
                    },
                "table-reference.dataset-id" => {
                        request_table_reference_init(&mut request);
                        request.table_reference.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "num-rows" => {
                        request_table_reference_init(&mut request);
                        request.num_rows = Some(value.unwrap_or("").to_string());
                    },
                "num-bytes" => {
                        request_table_reference_init(&mut request);
                        request.num_bytes = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_table_reference_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "friendly-name" => {
                        request_table_reference_init(&mut request);
                        request.friendly_name = Some(value.unwrap_or("").to_string());
                    },
                "expiration-time" => {
                        request_table_reference_init(&mut request);
                        request.expiration_time = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_table_reference_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_table_reference_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_table_reference_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "view.query" => {
                        request_view_init(&mut request);
                        request.view.as_mut().unwrap().query = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _tables_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Table::default();
        let mut call = self.hub.tables().update(&request, &self.opt.arg_project_id, &self.opt.arg_dataset_id, &self.opt.arg_table_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_table_reference_init(request: &mut api::Table) {
                if request.table_reference.is_none() {
                    request.table_reference = Some(Default::default());
                }
            }
            
            fn request_view_init(request: &mut api::Table) {
                if request.view.is_none() {
                    request.view = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "table-reference.project-id" => {
                        request_table_reference_init(&mut request);
                        request.table_reference.as_mut().unwrap().project_id = Some(value.unwrap_or("").to_string());
                    },
                "table-reference.table-id" => {
                        request_table_reference_init(&mut request);
                        request.table_reference.as_mut().unwrap().table_id = Some(value.unwrap_or("").to_string());
                    },
                "table-reference.dataset-id" => {
                        request_table_reference_init(&mut request);
                        request.table_reference.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "num-rows" => {
                        request_table_reference_init(&mut request);
                        request.num_rows = Some(value.unwrap_or("").to_string());
                    },
                "num-bytes" => {
                        request_table_reference_init(&mut request);
                        request.num_bytes = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_table_reference_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "friendly-name" => {
                        request_table_reference_init(&mut request);
                        request.friendly_name = Some(value.unwrap_or("").to_string());
                    },
                "expiration-time" => {
                        request_table_reference_init(&mut request);
                        request.expiration_time = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_table_reference_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_table_reference_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_table_reference_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "view.query" => {
                        request_view_init(&mut request);
                        request.view.as_mut().unwrap().query = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

        if self.opt.cmd_datasets {
            if self.opt.cmd_delete {
                call_result = self._datasets_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._datasets_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._datasets_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._datasets_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._datasets_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._datasets_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_jobs {
            if self.opt.cmd_get {
                call_result = self._jobs_get(dry_run, &mut err);
            } else if self.opt.cmd_get_query_results {
                call_result = self._jobs_get_query_results(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._jobs_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._jobs_list(dry_run, &mut err);
            } else if self.opt.cmd_query {
                call_result = self._jobs_query(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_projects {
            if self.opt.cmd_list {
                call_result = self._projects_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_tabledata {
            if self.opt.cmd_insert_all {
                call_result = self._tabledata_insert_all(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._tabledata_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_tables {
            if self.opt.cmd_delete {
                call_result = self._tables_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._tables_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._tables_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._tables_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._tables_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._tables_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }

        if dry_run {
            if err.issues.len() > 0 {
                err_opt = Some(err);
            }
        }
        (call_result, err_opt)
    }

    // Please note that this call will fail if any part of the opt can't be handled
    fn new(opt: Options) -> Result<Engine, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(&opt.flag_config_dir) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match cmn::application_secret_from_directory(&config_dir, "bigquery2-secret.json", 
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(  &secret, DefaultAuthenticatorDelegate,
                                        if opt.flag_debug_auth {
                                            hyper::Client::with_connector(mock::TeeConnector {
                                                    connector: hyper::net::HttpConnector(None) 
                                                })
                                        } else {
                                            hyper::Client::new()
                                        },
                                        JsonTokenStorage {
                                          program_name: "bigquery2",
                                          db_dir: config_dir.clone(),
                                        }, None);

        let client = 
            if opt.flag_debug {
                hyper::Client::with_connector(mock::TeeConnector {
                        connector: hyper::net::HttpConnector(None) 
                    })
            } else {
                hyper::Client::new()
            };
        let engine = Engine {
            opt: opt,
            hub: api::Bigquery::new(client, auth),
        };

        match engine._doit(true) {
            (_, Some(err)) => Err(err),
            _ => Ok(engine),
        }
    }

    // Execute the call with all the bells and whistles, informing the caller only if there was an error.
    // The absense of one indicates success.
    fn doit(&self) -> Option<api::Error> {
        self._doit(false).0
    }
}

fn main() {
    let opts: Options = Options::docopt().decode().unwrap_or_else(|e| e.exit());
    match Engine::new(opts) {
        Err(err) => {
            writeln!(io::stderr(), "{}", err).ok();
            env::set_exit_status(err.exit_code);
        },
        Ok(engine) => {
            if let Some(err) = engine.doit() {
                writeln!(io::stderr(), "{:?}", err).ok();
                writeln!(io::stderr(), "{}", err).ok();
                env::set_exit_status(1);
            }
        }
    }
}