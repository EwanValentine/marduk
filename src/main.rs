extern crate rusoto_logs;

// use chrono::Utc;

use rusoto_core::Region;
use rusoto_logs::*;
use std::default::Default;
use std::option::Option;
use tokio::prelude::*;

macro_rules! query {
    ($lim:expr, $like:expr) => (
    format!("
        fields @timestamp, @message\
          | sort @timestamp desc\
          | limit {} \
          | filter @message like /{}/\
        ", $lim, $like));
}

#[tokio::main]
async fn main() {

    // The query string for the log groups, i.e partial string match
    // If you have a log group /aws/lambda/my-payment-function
    // You can match any using "payment"
    let cont = &"prem".to_string();

    // Query string for the log groups
    let query_string = query!(20, "error");

    let client = CloudWatchLogsClient::new(Region::EuWest1);
    let ls_lg = DescribeLogGroupsRequest{
        limit: None,
        log_group_name_prefix: Option::from(String::from("/aws/lambda")),
        next_token: None
    };
    let res = client.describe_log_groups(ls_lg).await;

    let mut found_log_groups: Vec<String> = vec![];
    match res {
        Ok(l) => {
            let lg = l.log_groups.unwrap();
            for l in lg {
                let lgn = l.log_group_name.unwrap();
                if lgn.contains(cont) {
                    found_log_groups.push(String::from(lgn));
               }
            }
        }
        _ => {}
    }

    println!("{:?}", found_log_groups);

    let query = StartQueryRequest{
        end_time: 1593692579,
        limit: None,
        query_string,
        log_group_names: Option::from(found_log_groups),
        log_group_name: None,
        start_time: 1593582579
    };

    let resp = client.start_query(query).await;
    let query_id = match resp {
        Ok(r) => {
            r.query_id.unwrap()
        },
        Err(e) => {
            println!("{}", e);
            String::from("")
        }
    };

    let get_query_results = GetQueryResultsRequest{
        query_id,
    };

    let mut status = "Running".to_string();
    let mut results: Option<Vec<Vec<ResultField>>> = Option::from(vec![]);
    while status == "Running" {
        let results_resp = client.get_query_results(get_query_results.clone()).await;
        match results_resp {
            Ok(r) => {
                status = r.status.unwrap();
                println!("Status: {:?}", status);
                if status == String::from("Complete") {
                    println!("Complete");
                    results = Option::from(r.results.unwrap());
                }
            }
            Err(e) => { panic!(e); }
            _ => {}
        }
    }

    for r in results.unwrap() {
        for field in r {
            let val = field.value;
            println!("Field: {:?}", val.unwrap());
        }
    }
}
