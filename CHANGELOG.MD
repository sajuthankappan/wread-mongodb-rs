## 0.17.1

* Upgrade mongodb to 2.0.0

## 0.17.0

* Upgrade mongodb to 2.0.0-beta.3

## 0.15.0

* Add aggregate & count methods
* Breaking change: find method renamed to find_simple

## 0.14.0

* Rename crate name to wread-mongodb

## 0.12.4

* Add replace_one method & removed feature flags

## 0.12.3

* Change parameter type from String to &str in find_by_string_id

## 0.12.1

* Change parameter type from String to &str in find_one_by_string_field

## 0.12.1

* Fix find_one_and_replace

## 0.12.0

* Add find_one_and_replace and update dependent crate versions

## 0.11.0

* Add find_one_and_update

## 0.10.0

* Add find_all

## 0.9.0

* Upgrade mongodb to 1.0.0, and update to async functions using default mongo async runtime (tokio)

## 0.8.1

* Revert back to mongodb 0.9.2, but retaining async functions

## 0.8.0

* Upgrade mongodb to 0.10.0 and move to async functions

## 0.7.0

* Add find_by_id, find_by_string_id, find_one_by_string_field, find_by_string_field method
* Re-export of bson

## 0.6.1

* Replace returning errors as Box<dyn std::error::Error + Send + Sync + 'static> to mongodb::error::Error

## 0.6.0

* Add find_with_sort method

## 0.1.0

* First preview release