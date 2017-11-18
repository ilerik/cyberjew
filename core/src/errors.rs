use std::fmt;
use std::fs::File;
use log::SetLoggerError as LoggerError;
use system::LaunchStage as LaunchStage;
//use co::Error as CoasterError;
use toml::de::Error as TOMLError;

// Create the Error, ErrorKind, ResultExt, and Result types
error_chain! { 
    foreign_links {
        StdIo(::std::io::Error);
        Hyper(::hyper::Error);
        Utf8(::std::str::Utf8Error);
        AddrParse(::std::net::AddrParseError);
        LoggerError(LoggerError);
        TOMLError(TOMLError);
        //CoasterError(CoasterError);
    }

    errors {
        Launch(phase: LaunchStage) {
            description("An error occurred during startup")
            display("Startup aborted: {:?} did not complete successfully", phase)
        }

        ConfigLoad(path: String) {
            description("Config file not found")
            display("Unable to read file `{}`", path)
        }
    }
}

impl From<LaunchStage> for ErrorKind {
    fn from(v: LaunchStage) -> Self {
        ErrorKind::Launch(v)
    }
}