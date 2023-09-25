use crate::config::Config;
use crate::vault::clear_staging;
use std::fs;

/// Clean staging (ie. remove all the files from the staging area)
pub fn clear() {
    clear_staging();
}
