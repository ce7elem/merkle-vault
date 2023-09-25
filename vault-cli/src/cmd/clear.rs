use crate::vault::clear_staging;

/// Clean staging (ie. remove all the files from the staging area)
pub fn clear() {
    clear_staging();
}
