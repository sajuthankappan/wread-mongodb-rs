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