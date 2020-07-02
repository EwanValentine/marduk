# Marduk

A Cloudwatch Logs Grepper

## Rationale
The UI is probably my least favourite aspect of AWS, it's kinda clunky and un-intuitive in places.

One place in particular, is trying to hunt down error logs. Logs are divided into log streams, and it can be tricky, or time consuming to quickly view a series of related logs.

Cloudwatch Insights gives us a slightly simpler way of managing that process, and even a really powerful query language, too. However I still found that the UI got in the way for the most part.

This project attempts to wrap these insights queries, and to use the SDK to run them locally, and to display them right in your terminal. One major problem it solves, is you can search by log groups, in order to quickly select a series of related log groups. Whereas in the UI, you'd have to search, then click on each one. With Marduk, you just include a keyword, it will find, and select all of those log groups for you automatically.

## Install
Clone this repo, then run `$ cargo install --path .`.

## TODO
- [ ] - Parse JSON in response to nice format
- [ ] - Remove hard-coded search fields/values
- [ ] - Add option to include a query file:
    ```
        // query.marduk
        fields @timestamp, @message\
          | sort @timestamp desc\
          | limit {} \
          | filter @message like /ERROR/\
    ``` 
    So that you can just run it straight from your repo, and customise the query for each project.